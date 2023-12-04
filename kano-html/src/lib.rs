mod tag;

pub mod attr;
pub mod properties;

pub use tag::*;

#[derive(Clone, Copy)]
pub struct Element<T, C> {
    pub name: &'static str,
    pub props: T,
    pub children: C,
}

impl<T, C> Element<T, C> {
    pub const fn new(name: &'static str, props: T, children: C) -> Self {
        Self {
            name,
            props,
            children,
        }
    }
}

#[derive(kano::FromProperty)]
pub enum HtmlAttributes {
    Attribute(properties::Property),
    Event(kano::attr::On<kano::attr::Event>),
}
