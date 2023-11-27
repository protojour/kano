use crate::{
    platform::{Cursor, Platform},
    Attr, Diff,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Click,
    MouseOver,
}

pub struct On {
    event: Event,
    func: Box<dyn Fn()>,
}

impl On {
    pub fn click(func: impl Fn() + 'static) -> Self {
        Self::new(Event::Click, func)
    }

    pub fn mouseover(func: impl Fn() + 'static) -> Self {
        Self::new(Event::MouseOver, func)
    }

    fn new(event: Event, func: impl Fn() + 'static) -> Self {
        Self {
            event,
            func: Box::new(func),
        }
    }

    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn invoke(&self) {
        (self.func)();
    }
}

impl<P: Platform> Diff<P> for On {
    type State = Option<<P::Cursor as Cursor>::EventHandle>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        Some(cursor.on_event(self))
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        drop(state.take());
        *state = Some(cursor.on_event(self));
    }
}

impl<P: Platform> Attr<P> for On {}
