use std::borrow::Cow;

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
