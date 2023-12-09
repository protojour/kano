use std::borrow::Cow;

#[derive(PartialEq)]
pub struct Property {
    pub(crate) name: &'static str,
    pub(crate) value: PropertyValue,
}

impl Property {
    pub const fn new(name: &'static str, value: PropertyValue) -> Self {
        Self { name, value }
    }
}

#[derive(PartialEq)]
pub enum PropertyValue {
    String(Cow<'static, str>),
    Bool(bool),
    Number(i32),
    CommaSep(Vec<Cow<'static, str>>),
    SpaceSep(Vec<Cow<'static, str>>),
}

pub struct Strings(pub(crate) Vec<Cow<'static, str>>);

impl<const N: usize> From<[&'static str; N]> for Strings {
    fn from(value: [&'static str; N]) -> Self {
        Self(value.into_iter().map(Cow::Borrowed).collect())
    }
}

impl From<&'static str> for Strings {
    fn from(value: &'static str) -> Self {
        Self(vec![value.into()])
    }
}

#[derive(PartialEq)]
pub struct XmlProperty {
    pub(crate) namespace: XmlNamespace,
    pub(crate) name: &'static str,
    pub(crate) value: Cow<'static, str>,
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum XmlNamespace {
    Xlink,
}

impl XmlNamespace {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Xlink => "http://www.w3.org/1999/xlink",
        }
    }
}
