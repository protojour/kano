//! Kano is a work-in-progress GUI application framework written for and in Rust.
pub mod platform;
pub mod prelude;
pub mod reactive;
pub mod view;

#[cfg(feature = "vdom")]
pub mod vdom;

mod event;
mod registry;
mod signal;
mod style;
mod view_id;

use std::{cell::RefCell, convert::Infallible, marker::PhantomData, rc::Rc};

pub use event::*;
pub use kano_macros::view;
pub use kano_macros::Attribute;
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

pub trait Props<T> {
    type Iterator<'a>: Iterator<Item = &'a mut Option<T>>
    where
        T: 'a,
        Self: 'a;

    fn mut_iterator(&mut self) -> Self::Iterator<'_>;
}

pub trait Attribute<T> {
    fn into_prop(self) -> Option<T>;
}

/// A type used to signal that a type accepts no properties.
///
/// ```rust
/// fn component(_props: impl Props<Empty>) {}
/// ```
pub type Empty = Infallible;

impl<T, const N: usize> Props<T> for [Option<T>; N] {
    type Iterator<'a> = core::slice::IterMut<'a, Option<T>> where T: 'a;

    fn mut_iterator(&mut self) -> Self::Iterator<'_> {
        self.iter_mut()
    }
}

impl<T, A: Attribute<T>> Attribute<T> for Option<A> {
    fn into_prop(self) -> Option<T> {
        self.and_then(A::into_prop)
    }
}

pub struct Init<P> {
    platform: PhantomData<P>,
    context: PlatformContext,
}

thread_local! {
    #[allow(clippy::type_complexity)]
    pub(crate) static LOGGER: RefCell<Rc<dyn Fn(&str)>> = RefCell::new(Rc::new(|_| {}));
}

pub fn init<P: Platform>() -> Init<P> {
    let context = P::init(Box::new(signal::dispatch_pending_signals));

    LOGGER.with_borrow_mut({
        let context_logger = context.logger.clone();
        |logger| {
            *logger = context_logger;
        }
    });

    REGISTRY.with_borrow_mut({
        let on_signal_tick = context.on_signal_tick.clone();
        move |registry| {
            registry.platform_on_signal_tick = Some(on_signal_tick);
        }
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
    LOGGER.with_borrow(|logger| {
        logger(s);
    });
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

#[macro_export]
macro_rules! let_props {
    ({ $( $($seg:ident)::+ ($var:tt) $(,)?)+ } = $props:ident) => {
        $(
            let_props!(let mut $var);
        )+

        for prop in $props.mut_iterator() {
            match prop.take() {
                $(
                    Some($($seg)::+(binding)) => {
                        let_props!($var = binding);
                    }
                ),+
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
        drop($props);
    };
    (let mut $var:ident) => {
        let mut $var = None;
    };
    (let mut [$var:ident]) => {
        let mut $var = Vec::new();
    };
    ($var:ident = $expr:expr) => {
        $var = Some($expr);
    };
    ([$var:ident] = $expr:expr) => {
        $var.push($expr);
    };
}
