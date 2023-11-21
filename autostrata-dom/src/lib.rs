use js_sys::wasm_bindgen::JsCast;
use web_sys::{window, Document};

use autostrata::{Diff, Event, Handle, On, Platform};

pub struct Dom;

impl Dom {
    pub fn hydrate(tree: impl Diff) {
        let mut cursor = Cursor::LastChildOf(document().body().unwrap().into());
        let _state = tree.init::<Self>(&mut cursor);
    }
}

impl Platform for Dom {
    type Cursor = Cursor;

    fn new_text(text: &str, cursor: &mut Self::Cursor) -> Handle {
        let text_node = document().create_text_node(text);
        cursor.append(&text_node);
        Handle::DomNode(text_node.into())
    }

    fn update_text(handle: &mut Handle, text: &str) {
        let text_node: web_sys::Text = dom_node(handle).clone().dyn_into().unwrap();
        text_node.set_node_value(Some(text));
    }

    fn new_element(cursor: &mut Self::Cursor, name: &str) -> Handle {
        let element = document().create_element(name).unwrap();
        cursor.append(&element);
        Handle::DomNode(element.into())
    }

    fn register_event(cursor: &mut Self::Cursor, on_event: &On) -> Handle {
        match cursor {
            Cursor::AttrsOf(element) => {
                let event_type = match on_event.event() {
                    Event::Click => "onclick",
                    Event::MouseOver => "onmouseover",
                };

                let rust_fn: Box<dyn FnMut()> = Box::new(|| panic!("vfdsfds"));
                let js_closure = wasm_bindgen::closure::Closure::wrap(rust_fn);

                element
                    .add_event_listener_with_callback(
                        event_type,
                        js_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                Handle::DomAttr(event_type)
            }
            Cursor::LastChildOf(_) => panic!(),
        }
    }

    fn enter_child(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::LastChildOf(element) => {
                let last_child = element.last_element_child().unwrap();
                *cursor = Cursor::LastChildOf(last_child);
            }
            Cursor::AttrsOf(_) => {}
        }
    }
    fn exit_child(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::LastChildOf(element) => {
                let parent = element.parent_element().unwrap();
                *cursor = Cursor::LastChildOf(parent);
            }
            Cursor::AttrsOf(_) => {}
        }
    }

    fn enter_attrs(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::LastChildOf(element) => {
                let last_child = element.last_element_child().unwrap();
                *cursor = Cursor::AttrsOf(last_child);
            }
            Cursor::AttrsOf(_) => panic!(),
        }
    }

    fn exit_attrs(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::AttrsOf(element) => {
                let parent = element.parent_element().unwrap();
                *cursor = Cursor::LastChildOf(parent);
            }
            Cursor::LastChildOf(_) => panic!(),
        }
    }

    fn unmount(handle: &mut Handle, cursor: &mut Cursor) {
        match (cursor, handle) {
            (Cursor::LastChildOf(_), Handle::DomNode(node)) => {
                node.parent_element().unwrap().remove_child(node).unwrap();
            }
            (Cursor::AttrsOf(element), Handle::DomAttr(name)) => {
                let _ = element.remove_attribute(name);
            }
            _ => panic!("Can't unmount"),
        }
    }
}

#[derive(Clone)]
pub enum Cursor {
    LastChildOf(web_sys::Element),
    AttrsOf(web_sys::Element),
}

impl Cursor {
    fn append(&self, node: &web_sys::Node) {
        match self {
            Self::LastChildOf(element) => {
                element.append_child(node).unwrap();
            }
            Self::AttrsOf(_) => panic!(),
        }
    }
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}

#[inline]
fn dom_node(handle: &Handle) -> &web_sys::Node {
    match handle {
        Handle::DomNode(node) => node,
        _ => panic!(),
    }
}
