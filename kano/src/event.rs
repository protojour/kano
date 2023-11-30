use std::{fmt::Debug, rc::Rc};

use crate::{
    platform::{Cursor, Platform},
    Attr, Diff,
};

pub mod on {
    use super::*;

    pub fn click(func: impl Fn() + 'static) -> OnEvent {
        OnEvent::new(Event::Click, func)
    }

    pub fn mouseover(func: impl Fn() + 'static) -> OnEvent {
        OnEvent::new(Event::MouseOver, func)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Click,
    MouseOver,
}

#[derive(Clone)]
pub struct OnEvent {
    event: Event,
    func: Rc<dyn Fn()>,
}

impl Debug for OnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("On").field("event", &self.event).finish()
    }
}

impl OnEvent {
    fn new(event: Event, func: impl Fn() + 'static) -> Self {
        Self {
            event,
            func: Rc::new(func),
        }
    }

    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn invoke(&self) {
        (self.func)();
    }
}

impl<P: Platform> Diff<P> for OnEvent {
    type State = Option<<P::Cursor as Cursor>::EventHandle>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        Some(cursor.on_event(self))
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        drop(state.take());
        *state = Some(cursor.on_event(self));
    }
}

impl<P: Platform> Attr<P> for OnEvent {}
