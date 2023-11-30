use kano::Diff;

use std::{borrow::Cow, collections::HashMap};

use crate::{Web, WebCursor};

pub enum HtmlProp {
    OnEvent(kano::OnEvent),
    String(&'static str, Cow<'static, str>),
    Bool(&'static str, bool),
}

pub struct HtmlStrProp {
    name: &'static str,
    value: Cow<'static, str>,
}

impl HtmlStrProp {
    pub const fn new(name: &'static str, value: Cow<'static, str>) -> Self {
        Self { name, value }
    }
}

impl kano::Attribute<HtmlProp> for kano::OnEvent {
    fn into_prop(self) -> Option<HtmlProp> {
        Some(HtmlProp::OnEvent(self))
    }
}

impl kano::Attribute<HtmlProp> for HtmlStrProp {
    fn into_prop(self) -> Option<HtmlProp> {
        Some(HtmlProp::String(self.name, self.value))
    }
}

impl<const N: usize> Diff<Web> for [Option<HtmlProp>; N] {
    type State = (Self, HashMap<usize, gloo::events::EventListener>);

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        let mut listeners = HashMap::new();

        for (index, prop) in self.iter().enumerate() {
            match prop {
                Some(HtmlProp::OnEvent(on_event)) => {
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
                (Some(HtmlProp::OnEvent(on_event)), _) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                (None, Some(HtmlProp::OnEvent(_))) => {
                    // Listener was weirdly deleted
                    listeners.remove(&index);
                }
                _ => todo!(),
            }
        }
    }
}
