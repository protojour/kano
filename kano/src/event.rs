use std::{fmt::Debug, rc::Rc};

pub mod on {
    use super::*;

    pub fn click(func: impl Fn() + 'static) -> On<Click> {
        On::new(Click, func)
    }

    pub fn mouseover(func: impl Fn() + 'static) -> On<MouseOver> {
        On::new(MouseOver, func)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Click;

#[derive(Clone, Copy, Debug)]
pub struct MouseOver;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    Click,
    MouseOver,
}

#[derive(Clone)]
pub struct On<E> {
    event: E,
    func: Rc<dyn Fn()>,
}

impl<E: Debug> Debug for On<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("On").field("event", &self.event).finish()
    }
}

impl<E> On<E> {
    fn new(event: E, func: impl Fn() + 'static) -> Self {
        Self {
            event,
            func: Rc::new(func),
        }
    }

    pub fn event(&self) -> &E {
        &self.event
    }

    pub fn invoke(&self) {
        (self.func)();
    }
}

impl From<On<Click>> for On<Event> {
    fn from(value: On<Click>) -> Self {
        Self {
            event: Event::Click,
            func: value.func,
        }
    }
}

impl From<On<MouseOver>> for On<Event> {
    fn from(value: On<MouseOver>) -> Self {
        Self {
            event: Event::MouseOver,
            func: value.func,
        }
    }
}
