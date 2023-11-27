//! Kano is a work-in-progress GUI application framework written for and in Rust.
pub mod platform;
pub mod prelude;
pub mod reactive;
pub mod view;

mod event;
mod registry;
mod signal;
mod style;

pub use event::*;
pub use kano_macros::view;
use platform::{Cursor, Platform};
pub use style::*;

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

pub fn log(s: &str) {
    #[cfg(feature = "web")]
    web_util::log(s);
}

#[cfg(feature = "web")]
mod web_util {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
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
