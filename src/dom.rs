use js_sys::wasm_bindgen::JsCast;
use web_sys::{window, Document};

use crate::{Diff, Handle, Renderer};

pub struct Dom;

impl Dom {
    pub fn hydrate(tree: impl Diff) {
        let mut cursor = Cursor::LastChildOf(document().body().unwrap().into());
        let _state = tree.init::<Self>(&mut cursor);
    }
}

impl Renderer for Dom {
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

    fn enter_attrs(_cursor: &mut Self::Cursor) {}
    fn exit_attrs(_cursor: &mut Self::Cursor) {}

    fn enter_child(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::LastChildOf(element) => {
                let last_child = element.last_element_child().unwrap();
                *cursor = Cursor::LastChildOf(last_child);
            }
        }
    }
    fn exit_child(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::LastChildOf(element) => {
                let parent = element.parent_element().unwrap();
                *cursor = Cursor::LastChildOf(parent);
            }
        }
    }

    fn unmount(handle: &mut Handle) {
        let node = dom_node(handle);
        node.parent_element().unwrap().remove_child(node).unwrap();
    }
}

#[derive(Clone)]
pub enum Cursor {
    LastChildOf(web_sys::Element),
}

impl Cursor {
    fn append(&self, node: &web_sys::Node) {
        match self {
            Self::LastChildOf(element) => {
                element.append_child(node).unwrap();
            }
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
