mod tag;

pub mod attr;
pub mod properties;

use properties::Property;

pub use tag::*;

#[derive(Clone, Copy)]
pub struct SvgElement<A, C> {
    pub tag_name: &'static str,
    pub props: A,
    pub children: C,
}

impl<A, C> SvgElement<A, C> {
    pub const fn new(tag_name: &'static str, props: A, children: C) -> Self {
        Self {
            tag_name,
            props,
            children,
        }
    }
}

pub struct SvgAttribute(Property);
