use crate::{
    platform::{AttrHandle, Cursor, Platform},
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
    type State = OnState;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        OnState {
            handle: cursor.on_event(self),
        }
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl<P: Platform> Attr<P> for On {}

pub struct OnState {
    /// This is used to keep listeners alive
    #[allow(unused)]
    handle: AttrHandle,
}
