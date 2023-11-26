pub mod platform;
pub mod prelude;
pub mod reactive;
pub mod view;

mod event;
mod registry;
mod signal;
mod style;

pub use event::*;
use platform::{Cursor, ElementHandle, Platform};
pub use style::*;

pub use autostrata_macros::view;

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
        pub type $platform = autostrata_tui::Tui;

        /// The concrete view trait for this application.
        #[cfg(feature = "tui")]
        pub trait $view: autostrata::View<autostrata_tui::Tui> {}

        #[cfg(feature = "tui")]
        impl<V: autostrata::View<autostrata_tui::Tui>> $view for V {}

        /// Type alias for the current platform.
        #[cfg(feature = "web")]
        pub type $platform = autostrata_web::Web;

        /// The concrete view trait for this application.
        #[cfg(feature = "web")]
        pub trait $view: autostrata::View<autostrata_web::Web> {}

        #[cfg(feature = "web")]
        impl<V: autostrata::View<autostrata_web::Web>> $view for V {}
    };
}
