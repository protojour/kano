use kano::{Click, MouseOver};

use std::borrow::Cow;

#[derive(kano::Attribute)]
pub enum HtmlProperties {
    Attribute(HtmlProperty),
    Event(kano::On<kano::Event>),
}

impl kano::Attribute<HtmlProperties> for kano::On<Click> {
    fn into_prop(self) -> Option<HtmlProperties> {
        Some(HtmlProperties::Event(self.into()))
    }
}

impl kano::Attribute<HtmlProperties> for kano::On<MouseOver> {
    fn into_prop(self) -> Option<HtmlProperties> {
        Some(HtmlProperties::Event(self.into()))
    }
}

#[derive(PartialEq)]
pub struct HtmlProperty {
    pub idl_name: &'static str,
    pub value: HtmlPropertyValue,
}

impl HtmlProperty {
    pub const fn new(idl_name: &'static str, value: HtmlPropertyValue) -> Self {
        Self { idl_name, value }
    }
}

#[derive(PartialEq)]
pub enum HtmlPropertyValue {
    String(Cow<'static, str>),
    CommaSep(Vec<Cow<'static, str>>),
    SpaceSep(Vec<Cow<'static, str>>),
    Bool(bool),
    Number(i32),
    MaybeBool(Option<bool>),
}

pub struct Strings(pub(crate) Vec<Cow<'static, str>>);

impl<const N: usize> From<[&'static str; N]> for Strings {
    fn from(value: [&'static str; N]) -> Self {
        Self(value.into_iter().map(|str| Cow::Borrowed(str)).collect())
    }
}

pub enum StringOrBool {
    String(Cow<'static, str>),
    Bool(bool),
}

impl From<Cow<'static, str>> for StringOrBool {
    fn from(value: Cow<'static, str>) -> Self {
        Self::String(value)
    }
}

impl From<bool> for StringOrBool {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
