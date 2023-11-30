use std::{fmt::Debug, rc::Rc};

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
