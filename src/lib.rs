use std::any::Any;

mod element;
mod option;
mod text;
mod tuple;
mod unit;

#[cfg(feature = "dom")]
pub mod dom;

pub use element::*;

pub trait Diff {
    type State;

    // TODO: Should take renderer instance?
    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State;
    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor);
}

pub trait View: Diff {}

impl<T: Diff> View for T where T::State: Unmount {}

pub trait Platform {
    type Cursor: Clone + 'static;

    fn new_text(text: &str, cursor: &mut Self::Cursor) -> Handle;
    fn update_text(handle: &mut Handle, text: &str);

    fn new_element(cursor: &mut Self::Cursor, name: &str) -> Handle;

    fn enter_attrs(cursor: &mut Self::Cursor);
    fn exit_attrs(cursor: &mut Self::Cursor);

    fn enter_child(cursor: &mut Self::Cursor);
    fn exit_child(cursor: &mut Self::Cursor);

    fn unmount(handle: &mut Handle);
}

pub enum Handle {
    Index(usize),
    Dyn(Box<dyn Any>),
    #[cfg(feature = "dom")]
    DomNode(web_sys::Node),
}

pub trait Unmount: Sized {
    fn unmount<P: Platform>(&mut self);
}

impl Unmount for Handle {
    fn unmount<P: Platform>(&mut self) {
        P::unmount(self);
    }
}

impl<T> Unmount for (Handle, T) {
    fn unmount<P: Platform>(&mut self) {
        P::unmount(&mut self.0);
    }
}

pub trait List: Diff {}
