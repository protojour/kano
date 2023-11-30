use kano::Diff;

use std::{borrow::Cow, collections::HashMap};

use crate::{Web, WebCursor};

#[derive(kano::Attribute)]
pub enum HtmlProperties {
    Attribute(HtmlProperty),
    Event(kano::OnEvent),
}

#[derive(PartialEq)]
pub struct HtmlProperty {
    idl_name: &'static str,
    value: HtmlPropertyValue,
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

impl<const N: usize> Diff<Web> for [Option<HtmlProperties>; N] {
    type State = (Self, HashMap<usize, gloo::events::EventListener>);

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        let mut listeners = HashMap::new();

        for (index, prop) in self.iter().enumerate() {
            match prop {
                Some(HtmlProperties::Event(on_event)) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                Some(HtmlProperties::Attribute(property)) => {
                    set_html_attribute(cursor.get_element(), property);
                }
                _ => {}
            }
        }

        (self, listeners)
    }

    fn diff(self, (old_props, listeners): &mut Self::State, cursor: &mut WebCursor) {
        for (index, (new, state)) in self.into_iter().zip(old_props.iter_mut()).enumerate() {
            match (new, &state) {
                (Some(HtmlProperties::Event(on_event)), _) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                (None, Some(HtmlProperties::Event(_))) => {
                    // Listener was weirdly deleted
                    listeners.remove(&index);
                }
                (
                    Some(HtmlProperties::Attribute(property)),
                    Some(HtmlProperties::Attribute(old)),
                ) => {
                    if &property != old {
                        set_html_attribute(cursor.get_element(), &property);
                    }
                    *state = Some(HtmlProperties::Attribute(property));
                }
                (Some(HtmlProperties::Attribute(property)), None) => {
                    set_html_attribute(cursor.get_element(), &property);
                    *state = Some(HtmlProperties::Attribute(property));
                }
                (None, Some(HtmlProperties::Attribute(prop))) => {
                    cursor
                        .get_element()
                        .remove_attribute(prop.idl_name)
                        .unwrap();
                }
                _ => kano::log("TODO: Set other attribute"),
            }
        }
    }
}

fn set_html_attribute(element: &web_sys::Element, property: &HtmlProperty) {
    let name = property.idl_name;
    match &property.value {
        HtmlPropertyValue::String(string) => {
            element.set_attribute(name, string).unwrap();
        }
        HtmlPropertyValue::CommaSep(strings) => {
            let items = strings.iter().map(|s| -> &str { &*s }).collect::<Vec<_>>();
            element.set_attribute(name, &items.join(", ")).unwrap();
        }
        HtmlPropertyValue::SpaceSep(strings) => {
            let items = strings.iter().map(|s| -> &str { &*s }).collect::<Vec<_>>();
            element.set_attribute(name, &items.join(" ")).unwrap();
        }
        HtmlPropertyValue::Bool(bool) => {
            element.set_attribute(name, &format!("{bool}")).unwrap();
        }
        HtmlPropertyValue::Number(number) => {
            element.set_attribute(name, &format!("{number}")).unwrap();
        }
        HtmlPropertyValue::MaybeBool(value) => match value {
            Some(bool) => {
                element.set_attribute(name, &format!("{bool}")).unwrap();
            }
            None => {
                element.set_attribute(name, "").unwrap();
            }
        },
    }
}
