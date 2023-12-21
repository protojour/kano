//! Kano is a work-in-progress GUI application framework written for and in Rust.

pub mod attr;
pub mod history;
pub mod markup;
pub mod platform;
pub mod prelude;
pub mod property;
pub mod reactive;
pub mod view;

#[cfg(feature = "vdom")]
pub mod vdom;

#[cfg(feature = "routing")]
pub mod router;

mod registry;
mod signal;
mod view_id;

use std::{cell::RefCell, convert::Infallible, marker::PhantomData, rc::Rc};

pub use kano_macros::svg_view;
pub use kano_macros::view;
pub use kano_macros::FromProperty;
use platform::{Platform, PlatformContext, PlatformInit};
use registry::REGISTRY;
use view::Reactive;

/// A view is a UI node on a platform `P` defined by a markup language `M`.
pub trait View<P, M: markup::Markup<P>> {
    /// The state in a constant context
    type ConstState;

    /// The state in a diffable context
    type DiffState;

    /// Initialize the view without support for diffing.
    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState;

    /// Initialize the view with support for diffing.
    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState;

    /// Apply a diff.
    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor);
}

/// The children of a [View] on a platform `P` defined by a markup language `M`.
pub trait Children<P, M: markup::Markup<P>> {
    /// The state in a constant context
    type ConstState;

    /// The state in a diffable context
    type DiffState;

    /// Initialize the children without support for diffing.
    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState;

    /// Initialize the children with support for diffing.
    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState;

    /// Apply a diff.
    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor);
}

/// Perform diffing of a [View]'s properties on a platform `P` defined by a markup language `M`.
pub trait DiffProps<P, M: markup::Markup<P>> {
    /// The state in a constant context
    type ConstState;

    /// The state in a diffable context
    type DiffState;

    /// Initialize the props without support for diffing.
    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState;

    /// Initialize the props with support for diffing.
    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState;

    /// Apply a diff.
    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor);
}

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
/// use kano::*;
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
    let history_signal = REGISTRY.with_borrow(|registry| registry.globals.history_signal);

    let context = P::init(PlatformInit {
        signal_dispatch: Box::new(signal::dispatch_pending_signals),
        history_refresh: Rc::new(move || {
            history_signal.send();
        }),
    });

    let history_api = context.history_api.clone();

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
            registry.globals.history_api = history_api;
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
        V: View<P, P::Markup> + 'static,
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
        pub trait $view: kano::View<kano_tui::Tui, kano_tui::Tml> {}

        #[cfg(feature = "tui")]
        impl<V: kano::View<kano_tui::Tui, kano_tui::Tml>> $view for V {}

        /// Type alias for the current platform.
        #[cfg(feature = "web")]
        pub type $platform = kano_web::Web;

        /// The concrete view trait for this application.
        #[cfg(feature = "web")]
        pub trait $view: kano::View<kano_web::Web, kano_web::Html5> {}

        #[cfg(feature = "web")]
        impl<V: kano::View<kano_web::Web, kano_web::Html5>> $view for V {}
    };
}

#[macro_export]
macro_rules! platform_use {
    ($lib:ident as $ident:ident) => {
        #[cfg(feature = "tui")]
        use $lib::tui as $ident;

        #[cfg(feature = "web")]
        use $lib::web as $ident;
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
