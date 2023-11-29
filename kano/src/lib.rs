//! Kano is a work-in-progress GUI application framework written for and in Rust.
pub mod platform;
pub mod prelude;
pub mod reactive;
pub mod view;

mod event;
mod registry;
mod signal;
mod style;
mod view_id;

use std::marker::PhantomData;

pub use event::*;
pub use kano_macros::view;
use platform::{Cursor, Platform, PlatformContext};
use registry::REGISTRY;
pub use style::*;
use view::Func;

pub trait Diff<P: Platform> {
    type State;

    // TODO: Should take renderer instance?
    fn init(self, cursor: &mut P::Cursor) -> Self::State;
    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor);
}

pub trait View<P: Platform>: Diff<P> {}

pub trait Children<P: Platform>: Diff<P> {}

pub trait AttrSet<P: Platform>: Diff<P> {}

pub trait Attr<P: Platform>: Diff<P> {}

pub struct Init<P> {
    platform: PhantomData<P>,
    context: PlatformContext,
}

pub fn init<P: Platform>() -> Init<P> {
    let context = P::init(Box::new(signal::dispatch_pending_signals));
    let on_signal_tick = context.on_signal_tick.clone();
    let logger = context.logger.clone();

    REGISTRY.with_borrow_mut(move |registry| {
        registry.platform_on_signal_tick = Some(on_signal_tick);
        registry.logger = Some(logger);
    });

    Init {
        platform: PhantomData,
        context,
    }
}

impl<P: Platform> Init<P> {
    pub fn run_app<V>(self, func: impl (FnOnce() -> V) + 'static) -> anyhow::Result<()>
    where
        V: View<P> + 'static,
    {
        P::run(Func(func, ()), self.context)
    }
}

pub fn log(s: &str) {
    if let Some(logger) = REGISTRY.with_borrow(|registry| registry.logger.clone()) {
        logger(s)
    }
}

#[macro_export]
macro_rules! define_platform {
    ($platform:ident, $view:ident) => {
        /// Type alias for the current platform.
        #[cfg(feature = "tui")]
        pub type $platform = kano_tui::Tui;

        /// The concrete view trait for this application.
        #[cfg(feature = "tui")]
        pub trait $view: kano::View<kano_tui::Tui> {}

        #[cfg(feature = "tui")]
        impl<V: kano::View<kano_tui::Tui>> $view for V {}

        /// Type alias for the current platform.
        #[cfg(feature = "web")]
        pub type $platform = kano_web::Web;

        /// The concrete view trait for this application.
        #[cfg(feature = "web")]
        pub trait $view: kano::View<kano_web::Web> {}

        #[cfg(feature = "web")]
        impl<V: kano::View<kano_web::Web>> $view for V {}
    };
}

#[macro_export]
macro_rules! platform_use {
    ($lib:ident $($path:tt)*) => {
        #[cfg(feature = "tui")]
        use $lib::tui$($path)*;

        #[cfg(feature = "web")]
        use $lib::web$($path)*;
    };
}
