use crate::{Attr, Diff, Handle, Unmount};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Click,
    MouseOver,
}

pub struct OnEvent(pub(crate) Event);

pub fn on_click() -> OnEvent {
    OnEvent(Event::Click)
}

pub fn on_mouseover() -> OnEvent {
    OnEvent(Event::MouseOver)
}

impl Diff for OnEvent {
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

impl Attr for OnEvent {}

pub struct EventState {
    event: Event,
    handle: Handle,
}

impl Unmount for EventState {
    fn unmount<P: crate::Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.handle, cursor);
    }
}
