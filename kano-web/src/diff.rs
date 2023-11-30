use std::collections::HashMap;

use kano::{Children, Diff, Props, View};
use kano_html::{
    properties::{HtmlProperties, HtmlProperty, HtmlPropertyValue},
    Element,
};

use crate::{web_cursor::WebCursor, Web};

impl<T: Props<HtmlProperties> + Diff<Web>, C: Children<Web>> Diff<Web> for Element<T, C> {
    type State = State<T, C>;

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        let _ = cursor.element(self.name);
        let props = self.props.init(cursor);
        let children = self.children.init(cursor);

        State { props, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut crate::WebCursor) {
        self.props.diff(&mut state.props, cursor);
        self.children.diff(&mut state.children, cursor);
    }
}

impl<T: Props<HtmlProperties> + Diff<Web>, C: Children<Web>> View<Web> for Element<T, C> {}

pub struct State<T: Diff<Web>, C: Children<Web>> {
    props: T::State,
    children: C::State,
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
