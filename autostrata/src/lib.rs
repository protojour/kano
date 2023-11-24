pub mod platform;
pub mod reactive;
pub mod view;

mod event;
mod pubsub;
mod registry;
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
