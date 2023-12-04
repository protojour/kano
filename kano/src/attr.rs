use std::{borrow::Cow, fmt::Debug, rc::Rc};

/// This attributes represents the target of a hyperlink.
#[derive(Clone, Debug)]
pub struct To(pub Cow<'static, str>);

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
    pub(crate) fn new(event: E, func: Rc<dyn Fn()>) -> Self {
        Self { event, func }
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
        Self::new(Event::Click, value.func)
    }
}

impl From<On<MouseOver>> for On<Event> {
    fn from(value: On<MouseOver>) -> Self {
        Self::new(Event::MouseOver, value.func)
    }
}
