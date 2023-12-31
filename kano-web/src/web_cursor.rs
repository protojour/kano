use gloo::events::EventListener;
use js_sys::wasm_bindgen::*;
use kano_svg::SvgMarkup;
use web_sys::{Element, EventTarget};

use kano::{
    attr::{Event, On},
    markup::NestMarkup,
};

use crate::{document, Html5, Svg1_1, Web};

#[derive(Clone, Debug)]
pub struct WebCursor {
    pub position: Position,
}

#[derive(Clone, Debug)]
pub enum Position {
    Detached,
    Node(web_sys::Node),
    AfterLastChild(web_sys::Element),
    EndOfShadowRoot(web_sys::ShadowRoot),
}

impl WebCursor {
    pub fn new_detached() -> Self {
        Self {
            position: Position::Detached,
        }
    }

    pub fn element(&mut self, tag: &str) -> web_sys::Node {
        let element = document().create_element(tag).unwrap();
        self.append_node(&element);
        // log(&format!("new element cursor: {cursor:?}"));
        element.into()
    }

    pub fn on_event(&mut self, on_event: On<Event>) -> EventListener {
        match &mut self.position {
            Position::Node(element) => {
                let event_target: &EventTarget = element.dyn_ref().unwrap();
                let event_type = match on_event.event() {
                    Event::Click => "click",
                    Event::MouseOver => "mouseover",
                };

                EventListener::new(event_target, event_type, move |_| {
                    on_event.invoke();
                })
            }
            Position::Detached => panic!(),
            Position::AfterLastChild(_) | Position::EndOfShadowRoot(_) => panic!(),
        }
    }

    pub fn get_element(&self) -> &web_sys::Element {
        match &self.position {
            Position::Node(node) => node.dyn_ref().unwrap(),
            _ => panic!(),
        }
    }
}

impl kano::markup::Cursor for WebCursor {
    type TextHandle = web_sys::Node;
    type EventHandle = gloo::events::EventListener;

    fn from_text_handle(handle: &web_sys::Node) -> Self {
        Self {
            position: Position::Node(handle.clone()),
        }
    }

    fn empty(&mut self) {
        let comment = document().create_comment("");
        self.append_node(&comment);
    }

    fn text(&mut self, text: &str) -> web_sys::Node {
        let text_node = document().create_text_node(text);
        self.append_node(&text_node);
        text_node.into()
    }

    fn update_text(&mut self, text: &str) {
        match &self.position {
            Position::Node(node) => {
                node.set_node_value(Some(text));
            }
            _ => panic!(),
        }
    }

    fn enter_children(&mut self) {
        match &mut self.position {
            Position::Node(node) => {
                if let Some(child) = node.first_child() {
                    // kano::log(&format!("enter children: had child {child:?}"));
                    self.position = Position::Node(child);
                } else if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    // kano::log(&format!("enter children: had no children"));
                    self.position = Position::AfterLastChild(element.clone());
                } else {
                    panic!();
                }
            }
            Position::AfterLastChild(_) | Position::Detached | Position::EndOfShadowRoot(_) => {
                panic!("Enter empty children {:?}", self.position);
            }
        }
    }

    fn exit_children(&mut self) {
        match &mut self.position {
            Position::Node(node) => {
                let parent = node.parent_element().unwrap();
                self.position = Position::Node(parent.dyn_into().unwrap());
            }
            Position::AfterLastChild(element) => {
                self.position = Position::Node(element.clone().into());
            }
            Position::EndOfShadowRoot(shadow_root) => {
                self.position = Position::Node(shadow_root.clone().into());
            }
            Position::Detached => panic!("no children"),
        }
    }

    fn next_sibling(&mut self) {
        match &mut self.position {
            Position::AfterLastChild(_) | Position::EndOfShadowRoot(_) => {}
            Position::Detached => panic!(),
            Position::Node(node) => {
                if let Some(next) = node.next_sibling() {
                    self.position = Position::Node(next);
                } else {
                    let parent_node = node.parent_node().unwrap();
                    match parent_node.dyn_into::<web_sys::Element>() {
                        Ok(parent_element) => {
                            self.position = Position::AfterLastChild(parent_element);
                        }
                        Err(parent_node) => {
                            self.position = Position::EndOfShadowRoot(
                                parent_node.dyn_into::<web_sys::ShadowRoot>().unwrap(),
                            );
                        }
                    }
                }
            }
        }
    }

    fn remove(&mut self) {
        match &mut self.position {
            Position::AfterLastChild(_) | Position::EndOfShadowRoot(_) => panic!(),
            Position::Detached => panic!(),
            Position::Node(node) => {
                let next = if let Some(next) = node.next_sibling() {
                    Position::Node(next)
                } else {
                    let parent = node.parent_element().unwrap();
                    Position::AfterLastChild(parent)
                };

                if let Some(element) = node.dyn_ref::<Element>() {
                    element.remove();
                } else {
                    let parent = node.parent_element().unwrap();
                    parent.remove_child(node).unwrap();
                }

                self.position = next;
            }
        }
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let mut replacement_cursor = WebCursor {
            position: Position::Detached,
        };
        func(&mut replacement_cursor);

        match (&self.position, replacement_cursor.position) {
            (Position::Detached, _) => {}
            (Position::Node(node), Position::Node(replacement)) => {
                let parent = node.parent_element().unwrap();
                parent.replace_child(&replacement, node).unwrap();

                self.position = Position::Node(replacement);
            }
            (Position::Node(_node), Position::Detached) => {
                panic!();
            }
            (Position::AfterLastChild(..), _) => {
                panic!();
            }
            _ => panic!(),
        }
    }
}

/// Ability for Html5 to transition into Svg markup
impl NestMarkup<Web, Svg1_1> for Html5 {
    type Nested = Svg1_1;

    fn nest(cursor: &mut Self::Cursor) -> WebCursor {
        cursor.clone()
    }

    fn unnest(nested: WebCursor, original: &mut Self::Cursor) {
        *original = nested;
    }
}

impl SvgMarkup<Web> for Svg1_1 {
    fn svg_element(tag_name: &'static str, cursor: &mut Self::Cursor) {
        let element = document()
            .create_element_ns(Some("http://www.w3.org/2000/svg"), tag_name)
            .unwrap();
        cursor.append_node(&element);
    }

    fn set_svg_attribute(name: &str, value: &str, cursor: &mut Self::Cursor) {
        cursor.get_element().set_attribute(name, value).unwrap();
    }

    fn remove_svg_attribute(name: &str, cursor: &mut Self::Cursor) {
        cursor.get_element().remove_attribute(name).unwrap();
    }

    fn set_xml_attribute(namespace: &str, name: &str, value: &str, cursor: &mut Self::Cursor) {
        cursor
            .get_element()
            .set_attribute_ns(Some(namespace), name, value)
            .unwrap();
    }

    fn remove_xml_attribute(namespace: &str, name: &str, cursor: &mut Self::Cursor) {
        cursor
            .get_element()
            .remove_attribute_ns(Some(namespace), name)
            .unwrap();
    }
}

impl WebCursor {
    fn append_node(&mut self, appendee: &web_sys::Node) {
        // log(&format!("append at Cursor: {self:?}"));
        match &mut self.position {
            Position::Detached => {
                self.position = Position::Node(appendee.clone());
            }
            Position::AfterLastChild(element) => {
                // log("append at empty");
                element.append_child(appendee).expect("A");
                self.position = Position::Node(appendee.clone());
            }
            Position::Node(node) => {
                // log("append after node");
                node.parent_element()
                    .expect("parent element of node")
                    .insert_before(appendee, node.next_sibling().as_ref())
                    .expect("insert_before");
                self.position = Position::Node(appendee.clone());
            }
            Position::EndOfShadowRoot(_) => {
                panic!()
            }
        }
    }
}
