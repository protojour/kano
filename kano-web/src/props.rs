use kano::Diff;

use std::{borrow::Cow, collections::HashMap};

use crate::{Web, WebCursor};

#[derive(kano::Attribute)]
pub enum HtmlProperties {
    OnEvent(kano::OnEvent),
    String(HtmlValueProp<Cow<'static, str>>),
    MultiString(HtmlValueProp<Vec<Cow<'static, str>>>),
    Bool(HtmlValueProp<bool>),
    Number(HtmlValueProp<i32>),
    MaybeBool(HtmlValueProp<Option<bool>>),
    StringOrBool(HtmlValueProp<StringOrBool>),
}

pub struct HtmlValueProp<T> {
    attr_name: &'static str,
    value: T,
}

impl<T> HtmlValueProp<T> {
    pub const fn new(attr_name: &'static str, value: T) -> Self {
        Self { attr_name, value }
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
                Some(HtmlProperties::OnEvent(on_event)) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                _ => {}
            }
        }

        (self, listeners)
    }

    fn diff(self, (old_props, listeners): &mut Self::State, cursor: &mut WebCursor) {
        for (index, (new, old)) in self.into_iter().zip(old_props.iter_mut()).enumerate() {
            match (new, old) {
                (Some(HtmlProperties::OnEvent(on_event)), _) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                (None, Some(HtmlProperties::OnEvent(_))) => {
                    // Listener was weirdly deleted
                    listeners.remove(&index);
                }
                _ => todo!(),
            }
        }
    }
}
