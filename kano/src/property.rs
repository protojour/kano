use std::borrow::Cow;

/// Constructs a new property of the [crate::attr::To] attribute.
pub fn to(location: impl Into<Cow<'static, str>>) -> crate::attr::To {
    crate::attr::To(location.into())
}

/// Various event handler properties
pub mod on {
    use std::rc::Rc;

    use crate::attr::*;

    pub fn click(func: impl Fn() + 'static) -> On<Click> {
        On::new(Click, Rc::new(func))
    }

    pub fn mouseover(func: impl Fn() + 'static) -> On<MouseOver> {
        On::new(MouseOver, Rc::new(func))
    }
}
