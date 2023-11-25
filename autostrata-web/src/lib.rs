#![allow(non_snake_case, non_upper_case_globals)]

use autostrata::platform::{AttrHandle, ElementHandle, Platform};
use gloo::events::EventListener;
use js_sys::wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use web_sys::{window, Document};

use autostrata::{Diff, Event, On, View};

pub mod html;

mod element;

#[cfg(feature = "web-component")]
pub mod web_component;

mod js {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

pub struct Web {}

impl Platform for Web {
    type Cursor = WebCursor;

    fn run_app<V: View<Self>, F: (FnOnce() -> V) + 'static>(func: F) {
        console_error_panic_hook::set_once();

        // let mut cursor = Cursor::EmptyChildrenOf(document().body().unwrap().into());
        let mut cursor = WebCursor::Detached;
        let state = autostrata::view::Func(func).init(&mut cursor);

        let WebCursor::Node(node) = cursor else {
            panic!("No node rendered");
        };

        document().body().unwrap().append_child(&node).unwrap();

        // Need to keep the initial state around, it keeps EventListeners alive
        std::mem::forget(state);
    }

    fn log(s: &str) {
        js::log(s);
    }

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(task);
    }
}

#[derive(Clone, Debug)]
pub enum WebCursor {
    Detached,
    Node(web_sys::Node),
    EmptyChildrenOf(web_sys::Element),
    AttrsOf(web_sys::Element),
}

impl autostrata::platform::Cursor for WebCursor {
    fn from_element_handle(handle: &ElementHandle) -> Self {
        match handle {
            ElementHandle::DomNode(node) => Self::Node(node.clone()),
        }
    }

    fn empty(&mut self) {
        match &self {
            WebCursor::AttrsOf(_) => {}
            _ => {
                let comment = document().create_comment("");
                self.append_node(&comment);
            }
        }
    }

    fn text(&mut self, text: &str) -> ElementHandle {
        let text_node = document().create_text_node(text);
        self.append_node(&text_node);
        js::log(&format!("new text node cursor: {self:?}"));
        ElementHandle::DomNode(text_node.into())
    }

    fn update_text(&mut self, text: &str) {
        match self {
            Self::Node(node) => {
                node.set_node_value(Some(text));
            }
            _ => panic!(),
        }
    }

    fn element(&mut self, name: &str) -> ElementHandle {
        let element = document().create_element(name).unwrap();
        // log(&format!(
        //     "NEW ELEMENT first child: {:?}",
        //     element.first_child(),
        // ));
        self.append_node(&element);
        // log(&format!("new element cursor: {cursor:?}"));
        ElementHandle::DomNode(element.into())
    }

    fn on_event(&mut self, on_event: On) -> AttrHandle {
        match self {
            WebCursor::AttrsOf(element) => {
                let event_target: &EventTarget = element.dyn_ref().unwrap();
                let event_type = match on_event.event() {
                    Event::Click => "click",
                    Event::MouseOver => "mouseover",
                };

                AttrHandle::DomEvent(EventListener::new(&event_target, event_type, move |_| {
                    on_event.invoke();
                }))
            }
            WebCursor::Node(_) => panic!(),
            WebCursor::Detached => panic!(),
            WebCursor::EmptyChildrenOf(_) => panic!(),
        }
    }

    fn enter_children(&mut self) {
        match self {
            WebCursor::Node(node) => {
                if let Some(child) = node.first_child() {
                    // log(&format!("enter child: had child {child:?}"));
                    *self = WebCursor::Node(child);
                } else if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    // log(&format!("enter child: had no children"));
                    *self = WebCursor::EmptyChildrenOf(element.clone());
                } else {
                    panic!("No children");
                }
            }
            WebCursor::EmptyChildrenOf(_) | WebCursor::Detached => {
                panic!("Enter empty children");
            }
            WebCursor::AttrsOf(_) => {}
        }
    }

    fn exit_children(&mut self) {
        // log("exit child");
        match self {
            WebCursor::Node(node) => {
                let parent = node.parent_element().unwrap();
                *self = WebCursor::Node(parent.dyn_into().unwrap());
            }
            WebCursor::EmptyChildrenOf(element) => {
                *self = WebCursor::Node(element.dyn_ref::<web_sys::Node>().unwrap().clone());
            }
            WebCursor::AttrsOf(_) => {}
            WebCursor::Detached => panic!("no children"),
        }
    }

    fn enter_attrs(&mut self) {
        match self {
            WebCursor::Node(node) => {
                if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    *self = WebCursor::AttrsOf(element.clone());
                } else {
                    panic!("Non-element attributes");
                }
            }
            WebCursor::EmptyChildrenOf(_) => {
                panic!("Entering attrs of empty children");
            }
            WebCursor::AttrsOf(_) | WebCursor::Detached => panic!(),
        }
    }

    fn exit_attrs(&mut self) {
        match self {
            WebCursor::AttrsOf(element) => {
                *self = WebCursor::Node(element.dyn_ref::<web_sys::Node>().unwrap().clone());
            }
            WebCursor::EmptyChildrenOf(_) => panic!(),
            WebCursor::Node(_) => panic!(),
            WebCursor::Detached => panic!(),
        }
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let mut replacement_cursor = WebCursor::Detached;
        func(&mut replacement_cursor);

        match (&self, replacement_cursor) {
            (WebCursor::Detached, _) => {}
            (WebCursor::Node(node), WebCursor::Node(replacement)) => {
                let parent = node.parent_element().unwrap();
                parent.replace_child(&replacement, node).unwrap();

                *self = WebCursor::Node(replacement);
            }
            (WebCursor::Node(_node), WebCursor::Detached) => {
                panic!();
            }
            (WebCursor::AttrsOf(_el), _) => {
                panic!()
            }
            (WebCursor::EmptyChildrenOf(_), _) => {
                panic!();
            }
            _ => panic!(),
        }
    }
}

impl WebCursor {
    fn append_node(&mut self, appendee: &web_sys::Node) {
        // log(&format!("append at Cursor: {self:?}"));
        match self {
            Self::Detached => {
                *self = Self::Node(appendee.clone());
            }
            Self::EmptyChildrenOf(element) => {
                // log("append at empty");
                element.append_child(appendee).expect("A");
                *self = Self::Node(appendee.clone());
            }
            Self::Node(node) => {
                // log("append after node");
                node.parent_element()
                    .expect("parent element of node")
                    .insert_before(appendee, node.next_sibling().as_ref())
                    .expect("insert_before");
                *self = Self::Node(appendee.clone());
            }
            Self::AttrsOf(_) => panic!("append to attrs"),
        }
    }
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
