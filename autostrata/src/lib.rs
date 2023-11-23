pub mod platform;
pub mod view;

mod event;
mod pubsub;
mod style;
mod use_state;

pub use event::*;
use platform::{Cursor, ElementHandle, Platform};
pub use style::*;
pub use use_state::*;

pub use autostrata_macros::view;

pub trait Diff {
    type State;

    // TODO: Should take renderer instance?
    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State;
    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor);
}

pub trait View: Diff {}

pub trait ViewState {}

impl<T: Diff> View for T where T::State: ViewState {}

pub trait Children: Diff {}

pub trait AttrSet: Diff {}

pub trait Attr: Diff {}

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
