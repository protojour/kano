//! Kano is a work-in-progress GUI application framework written for and in Rust.
pub mod platform;
pub mod prelude;
pub mod reactive;
pub mod view;

#[cfg(feature = "vdom")]
pub mod vdom;

#[cfg(feature = "routing")]
pub mod router;

pub mod history;

mod event;
mod registry;
mod signal;
mod style;
mod view_id;

use std::{cell::RefCell, convert::Infallible, marker::PhantomData, rc::Rc};

pub use event::*;
pub use kano_macros::view;
pub use kano_macros::FromProperty;
use platform::{Cursor, Platform, PlatformContext};
use registry::REGISTRY;
pub use style::*;
use view::Reactive;

/// Kano's core trait for view diffing.
pub trait Diff<P: Platform> {
    type State;

    // TODO: Should take renderer instance?
    fn init(self, cursor: &mut P::Cursor) -> Self::State;
    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor);
}

/// A marker trait for views.
pub trait View<P: Platform>: Diff<P> {}

/// A marker trait for the children of a view.
pub trait Children<P: Platform>: Diff<P> {}

/// The Props trait marks properties passed into a view function.
///
/// Props are instances a set of valid attributes, indicated by the generic parameter `A`.
pub trait Props<A> {
    type Iterator<'a>: Iterator<Item = &'a mut Option<A>>
    where
        A: 'a,
        Self: 'a;

    fn mut_iterator(&mut self) -> Self::Iterator<'_>;
}

/// A trait for attribute casting.
///
/// The trait expresses that some property of the attribte `A` can be converted into the attribute `Self`.
pub trait FromProperty<A>: Sized {
    fn from_property(property: A) -> Option<Self>;
}

pub trait DeserializeAttribute: Sized + 'static {
    fn describe(names: &mut Vec<&'static str>);

    fn deserialize(name: &str, value: String) -> Option<Self>;
}

/// An empty set of attributes.
///
/// # Example
///
/// ```rust
/// fn component(_props: impl Props<Empty>) {}
/// ```
pub type Empty = Infallible;

impl<A, const N: usize> Props<A> for [Option<A>; N] {
    type Iterator<'a> = core::slice::IterMut<'a, Option<A>> where A: 'a;

    fn mut_iterator(&mut self) -> Self::Iterator<'_> {
        self.iter_mut()
    }
}

impl<A> Props<A> for Vec<Option<A>> {
    type Iterator<'a> = core::slice::IterMut<'a, Option<A>> where A: 'a;

    fn mut_iterator(&mut self) -> Self::Iterator<'_> {
        self.iter_mut()
    }
}

impl<A, B: FromProperty<A>> FromProperty<Option<A>> for B {
    fn from_property(attr: Option<A>) -> Option<B> {
        attr.and_then(Self::from_property)
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
            registry.initialized = true;
        }
    });

    Init {
        platform: PhantomData,
        context,
    }
}

impl<P: Platform> Init<P> {
    pub fn run_app<V>(self, func: impl (Fn() -> V) + 'static) -> anyhow::Result<()>
    where
        V: View<P> + 'static,
    {
        P::run(Reactive(func), self.context)
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
            $crate::_let_props_helper!(let mut $var);
        )+

        for prop in $props.mut_iterator() {
            match prop.take() {
                $(
                    Some($($seg)::+(binding)) => {
                        $crate::_let_props_helper!($var = binding);
                    }
                ),+
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
        drop($props);
    };
}

#[macro_export]
macro_rules! _let_props_helper {
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
