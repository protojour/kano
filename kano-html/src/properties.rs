use kano::attr::{Click, MouseOver, On};

use std::borrow::Cow;

use crate::HtmlAttribute;

impl kano::FromProperty<On<Click>> for HtmlAttribute {
    fn from_property(property: On<Click>) -> Option<Self> {
        Some(Self::Event(property.into()))
    }
}

impl kano::FromProperty<On<MouseOver>> for HtmlAttribute {
    fn from_property(property: On<MouseOver>) -> Option<Self> {
        Some(Self::Event(property.into()))
    }
}

#[derive(PartialEq)]
pub struct Property {
    pub idl_name: &'static str,
    pub value: PropertyValue,
}

impl Property {
    pub const fn new(idl_name: &'static str, value: PropertyValue) -> Self {
        Self { idl_name, value }
    }
}

#[derive(PartialEq)]
pub enum PropertyValue {
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
        Self(value.into_iter().map(Cow::Borrowed).collect())
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
