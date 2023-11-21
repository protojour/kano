use crate::{Attr, Diff, Handle, Unmount};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Click,
    MouseOver,
}

pub struct On(pub(crate) Event);

impl On {
    pub fn click() -> On {
        On(Event::Click)
    }

    pub fn mouseover() -> On {
        On(Event::MouseOver)
    }

    pub fn event(&self) -> &Event {
        &self.0
    }
}

impl Diff for On {
    type State = EventState;

    fn init<P: crate::Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let handle = P::register_event(cursor, &self);
        EventState {
            event: self.0,
            handle,
        }
    }

    fn diff<P: crate::Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {}
}

impl Attr for On {}

pub struct EventState {
    event: Event,
    handle: Handle,
}

impl Unmount for EventState {
    fn unmount<P: crate::Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.handle, cursor);
    }
}
