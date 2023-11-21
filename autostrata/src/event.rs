use crate::{Attr, Diff, Handle, Unmount};

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

impl Diff for On {
    type State = OnState;

    fn init<P: crate::Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        OnState {
            handle: P::register_event(cursor, self),
        }
    }

    fn diff<P: crate::Platform>(self, _state: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl Attr for On {}

pub struct OnState {
    handle: Handle,
}

impl Unmount for OnState {
    fn unmount<P: crate::Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.handle, cursor);
    }
}
