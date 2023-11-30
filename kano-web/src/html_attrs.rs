use std::borrow::Cow;

use crate::props::HtmlStrProp;

pub fn href(value: impl Into<Cow<'static, str>>) -> HtmlStrProp {
    HtmlStrProp::new("href", value.into())
}
