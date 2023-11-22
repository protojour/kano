mod either;
mod element;
mod event;
mod option;
mod pubsub;
mod reactive;
mod style;
mod text;
mod tuple;
mod unit;

pub mod platform;

pub use either::Either;
pub use element::*;
pub use event::*;
use platform::{Cursor, ElementHandle, Platform};
pub use reactive::Reactive;
pub use style::*;

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
