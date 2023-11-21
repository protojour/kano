use std::any::Any;

mod element;
mod event;
mod option;
mod style;
mod text;
mod tuple;
mod unit;

pub mod platform;

pub use element::*;
pub use event::*;
use platform::{Handle, Platform};
pub use style::*;

pub trait Diff {
    type State;

    // TODO: Should take renderer instance?
    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State;
    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor);
}

pub trait View: Diff {}

pub trait ViewState: Unmount {}

impl<T: Diff> View for T where T::State: ViewState {}

pub trait Unmount: Sized {
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor);
}

pub trait Children: Diff {}

pub trait AttrSet: Diff {}

pub trait Attr: Diff {}

impl Unmount for Handle {
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(self, cursor);
    }
}

impl<T> Unmount for (Handle, T) {
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.0, cursor);
    }
}
