pub mod html;

pub mod attr;
pub mod properties;

#[derive(Clone, Copy)]
pub struct HtmlElement<A, C> {
    pub tag_name: &'static str,
    pub props: A,
    pub children: C,
}

impl<A, C> HtmlElement<A, C> {
    pub const fn new(tag_name: &'static str, props: A, children: C) -> Self {
        Self {
            tag_name,
            props,
            children,
        }
    }
}

#[derive(kano::FromProperty)]
pub enum HtmlAttribute {
    Attribute(properties::Property),
    Event(kano::attr::On<kano::attr::Event>),
}
