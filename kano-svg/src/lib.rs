pub mod attr;
pub mod properties;
pub mod svg;

mod diff;

use properties::{Property, XmlProperty};

#[derive(Clone, Copy)]
pub struct SvgElement<A, C> {
    pub tag_name: &'static str,
    pub props: SvgProps<A>,
    pub children: C,
}

impl<A, C> SvgElement<A, C> {
    pub const fn new(tag_name: &'static str, props: A, children: C) -> Self {
        Self {
            tag_name,
            props: SvgProps(props),
            children,
        }
    }
}

pub enum SvgAttribute {
    Svg(Property),
    Xml(XmlProperty),
}

#[derive(Clone, Copy)]
pub struct SvgProps<A>(pub A);

impl kano::FromProperty<SvgAttribute> for SvgAttribute {
    fn from_property(property: SvgAttribute) -> Option<Self> {
        Some(property)
    }
}

pub trait SvgCursor: kano::platform::Cursor {
    fn svg_element(&mut self, tag_name: &'static str);
    fn set_svg_attribute(&mut self, name: &str, value: &str);
    fn remove_svg_attribute(&mut self, name: &str);
    fn set_xml_attribute(&mut self, namespace: &str, name: &str, value: &str);
    fn remove_xml_attribute(&mut self, namespace: &str, name: &str);
}

pub mod xmlns {
    use std::borrow::Cow;

    use crate::SvgAttribute;

    #[derive(Clone, Debug)]
    pub struct Ns(&'static str, pub Cow<'static, str>);

    pub fn xlink(location: impl Into<Cow<'static, str>>) -> Ns {
        Ns("xlink", location.into())
    }

    impl kano::FromProperty<Ns> for SvgAttribute {
        fn from_property(_property: Ns) -> Option<Self> {
            None
        }
    }
}

pub mod xml {
    use std::borrow::Cow;

    use crate::SvgAttribute;

    #[derive(Clone, Debug)]
    pub struct Space;

    /// A xlink:href property.
    pub fn space(_location: impl Into<Cow<'static, str>>) -> Space {
        Space
    }

    impl kano::FromProperty<Space> for SvgAttribute {
        fn from_property(_property: Space) -> Option<Self> {
            None
        }
    }
}

pub mod xlink {
    use std::borrow::Cow;

    use crate::{
        properties::{XmlNamespace, XmlProperty},
        SvgAttribute,
    };

    #[derive(Clone, Debug)]
    pub struct Href(pub Cow<'static, str>);

    /// A xlink:href property.
    pub fn href(location: impl Into<Cow<'static, str>>) -> Href {
        Href(location.into())
    }

    impl kano::FromProperty<Href> for SvgAttribute {
        fn from_property(property: Href) -> Option<Self> {
            Some(SvgAttribute::Xml(XmlProperty {
                namespace: XmlNamespace::Xlink,
                name: "href",
                value: property.0,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attr::{class, d, height, id, viewBox, width},
        svg::*,
        SvgCursor,
    };
    use kano::{platform::Platform, view, View};

    pub fn _test_svg<P: Platform>() -> impl View<P>
    where
        P::Cursor: SvgCursor,
    {
        view! {
            <svg id="icon" /*xmlns="http://www.w3.org/2000/svg"*/ width="32" height="32" viewBox="0 0 32 32">
                <defs>
                    <style>".cls-1 { fill: none; }"</style>
                </defs>
                <path d="M22.9961,30H9.0039a1.0022,1.0022,0,0,1-.821-1.5769l6.9977-9.9965a1,1,0,0,1,1.6388,0l6.9977,9.9965A1.0022,1.0022,0,0,1,22.9961,30ZM10.92,28H21.08L16,20.7439Z"/>
                <path d="M28,24H24V22h4V6H4V22H8v2H4a2.0021,2.0021,0,0,1-2-2V6A2.0021,2.0021,0,0,1,4,4H28a2.0021,2.0021,0,0,1,2,2V22A2.0021,2.0021,0,0,1,28,24Z"/>
                <rect id="_Transparent_Rectangle_" /*data_name="&lt;Transparent Rectangle&gt;"*/ class="cls-1" width="32" height="32"/>
            </svg>
        }
    }
}
