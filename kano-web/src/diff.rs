use std::collections::HashMap;

use kano::{Children, DiffProps, Props, View};
use kano_html::{
    properties::{Property, PropertyValue},
    HtmlAttribute, HtmlElement,
};

use crate::{web_cursor::WebCursor, Html5, Web};

impl<A: Props<HtmlAttribute> + DiffProps<Web, Html5>, C: Children<Web, Html5>> View<Web, Html5>
    for HtmlElement<A, C>
{
    type ConstState = (A::ConstState, C::ConstState);
    type DiffState = (A::DiffState, C::DiffState);

    fn init_const(self, cursor: &mut WebCursor) -> Self::ConstState {
        let _ = cursor.element(self.tag_name);
        let props = self.props.init_const(cursor);
        let children = self.children.init_const(cursor);

        (props, children)
    }

    fn init_diff(self, cursor: &mut WebCursor) -> Self::DiffState {
        let _ = cursor.element(self.tag_name);
        let props = self.props.init_diff(cursor);
        let children = self.children.init_diff(cursor);

        (props, children)
    }

    fn diff(self, (props, children): &mut Self::DiffState, cursor: &mut crate::WebCursor) {
        self.props.diff(props, cursor);
        self.children.diff(children, cursor);
    }
}

impl<const N: usize> DiffProps<Web, Html5> for [Option<HtmlAttribute>; N] {
    /// The responsibility of the ConstState is to own the EventListeners
    /// and keep them active as long as the element is visible:
    type ConstState = Vec<gloo::events::EventListener>;

    type DiffState = (Self, HashMap<usize, gloo::events::EventListener>);

    fn init_const(self, cursor: &mut WebCursor) -> Self::ConstState {
        let mut listeners = vec![];

        for prop in self.iter() {
            match prop {
                Some(HtmlAttribute::Event(on_event)) => {
                    listeners.push(cursor.on_event(on_event.clone()));
                }
                Some(HtmlAttribute::Attribute(property)) => {
                    set_html_attribute(cursor.get_element(), property);
                }
                _ => {}
            }
        }

        listeners
    }

    fn init_diff(self, cursor: &mut WebCursor) -> Self::DiffState {
        let mut listeners = HashMap::new();

        for (index, prop) in self.iter().enumerate() {
            match prop {
                Some(HtmlAttribute::Event(on_event)) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                Some(HtmlAttribute::Attribute(property)) => {
                    set_html_attribute(cursor.get_element(), property);
                }
                _ => {}
            }
        }

        (self, listeners)
    }

    fn diff(self, (old_props, listeners): &mut Self::DiffState, cursor: &mut WebCursor) {
        for (index, (new, state)) in self.into_iter().zip(old_props.iter_mut()).enumerate() {
            match (new, &state) {
                (Some(HtmlAttribute::Event(on_event)), _) => {
                    listeners.insert(index, cursor.on_event(on_event.clone()));
                }
                (None, Some(HtmlAttribute::Event(_))) => {
                    // Listener was weirdly deleted
                    listeners.remove(&index);
                }
                (Some(HtmlAttribute::Attribute(property)), Some(HtmlAttribute::Attribute(old))) => {
                    if &property != old {
                        set_html_attribute(cursor.get_element(), &property);
                    }
                    *state = Some(HtmlAttribute::Attribute(property));
                }
                (Some(HtmlAttribute::Attribute(property)), None) => {
                    set_html_attribute(cursor.get_element(), &property);
                    *state = Some(HtmlAttribute::Attribute(property));
                }
                (None, Some(HtmlAttribute::Attribute(prop))) => {
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

fn set_html_attribute(element: &web_sys::Element, property: &Property) {
    let name = property.idl_name;
    match &property.value {
        PropertyValue::String(string) => {
            element.set_attribute(name, string).unwrap();
        }
        PropertyValue::CommaSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            element.set_attribute(name, &items.join(", ")).unwrap();
        }
        PropertyValue::SpaceSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            element.set_attribute(name, &items.join(" ")).unwrap();
        }
        PropertyValue::Bool(bool) => {
            element.set_attribute(name, &format!("{bool}")).unwrap();
        }
        PropertyValue::Number(number) => {
            element.set_attribute(name, &format!("{number}")).unwrap();
        }
        PropertyValue::MaybeBool(value) => match value {
            Some(bool) => {
                element.set_attribute(name, &format!("{bool}")).unwrap();
            }
            None => {
                element.set_attribute(name, "").unwrap();
            }
        },
    }
}
