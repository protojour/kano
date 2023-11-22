use autostrata::platform::{Handle, Platform};
use gloo::events::EventListener;
use js_sys::wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use web_sys::{window, Document};

use autostrata::{Event, On, View};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Dom {}

impl Dom {
    pub fn hydrate<V: View>(view: V) {
        // let mut cursor = Cursor::EmptyChildrenOf(document().body().unwrap().into());
        let mut cursor = Cursor::Detached;
        let state = view.init::<Self>(&mut cursor);

        let Cursor::Node(node) = cursor else {
            panic!("No node rendered");
        };

        document().body().unwrap().append_child(&node).unwrap();

        // Need to keep the initial state around, it keeps EventListeners alive
        std::mem::forget(state);
    }
}

impl Platform for Dom {
    type Cursor = Cursor;

    fn mark_empty(cursor: &mut Self::Cursor) {
        match &cursor {
            Cursor::AttrsOf(_) => {}
            _ => {
                let comment = document().create_comment("");
                cursor.append_node(&comment);
            }
        }
    }

    fn new_text(text: &str, cursor: &mut Self::Cursor) -> Handle {
        let text_node = document().create_text_node(text);
        cursor.append_node(&text_node);
        log(&format!("new text node cursor: {cursor:?}"));
        Handle::DomNode(text_node.into())
    }

    fn update_text(handle: &mut Handle, text: &str) {
        let text_node: web_sys::Text = dom_node(handle).clone().dyn_into().unwrap();
        text_node.set_node_value(Some(text));
    }

    fn new_element(name: &str, cursor: &mut Self::Cursor) -> Handle {
        let element = document().create_element(name).unwrap();
        // log(&format!(
        //     "NEW ELEMENT first child: {:?}",
        //     element.first_child(),
        // ));
        cursor.append_node(&element);
        // log(&format!("new element cursor: {cursor:?}"));
        Handle::DomNode(element.into())
    }

    fn register_event(cursor: &mut Self::Cursor, on_event: On) -> Handle {
        match cursor {
            Cursor::AttrsOf(element) => {
                let event_target: &EventTarget = element.dyn_ref().unwrap();
                let event_type = match on_event.event() {
                    Event::Click => "click",
                    Event::MouseOver => "mouseover",
                };

                Handle::DomEvent(EventListener::new(&event_target, event_type, move |_| {
                    on_event.invoke();
                }))
            }
            Cursor::Node(_) => panic!(),
            Cursor::Detached => panic!(),
            Cursor::EmptyChildrenOf(_) => panic!(),
        }
    }

    fn enter_child(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::Node(node) => {
                if let Some(child) = node.first_child() {
                    // log(&format!("enter child: had child {child:?}"));
                    *cursor = Cursor::Node(child);
                } else if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    // log(&format!("enter child: had no children"));
                    *cursor = Cursor::EmptyChildrenOf(element.clone());
                } else {
                    panic!("No children");
                }
            }
            Cursor::EmptyChildrenOf(_) | Cursor::Detached => {
                panic!("Enter empty children");
            }
            Cursor::AttrsOf(_) => {}
        }
    }
    fn exit_child(cursor: &mut Self::Cursor) {
        // log("exit child");
        match cursor {
            Cursor::Node(node) => {
                let parent = node.parent_element().unwrap();
                *cursor = Cursor::Node(parent.dyn_into().unwrap());
            }
            Cursor::EmptyChildrenOf(element) => {
                *cursor = Cursor::Node(element.dyn_ref::<web_sys::Node>().unwrap().clone());
            }
            Cursor::AttrsOf(_) => {}
            Cursor::Detached => panic!("no children"),
        }
    }

    fn enter_attrs(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::Node(node) => {
                if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    *cursor = Cursor::AttrsOf(element.clone());
                } else {
                    panic!("Non-element attributes");
                }
            }
            Cursor::EmptyChildrenOf(_) => {
                panic!("Entering attrs of empty children");
            }
            Cursor::AttrsOf(_) | Cursor::Detached => panic!(),
        }
    }

    fn exit_attrs(cursor: &mut Self::Cursor) {
        match cursor {
            Cursor::AttrsOf(element) => {
                *cursor = Cursor::Node(element.dyn_ref::<web_sys::Node>().unwrap().clone());
            }
            Cursor::EmptyChildrenOf(_) => panic!(),
            Cursor::Node(_) => panic!(),
            Cursor::Detached => panic!(),
        }
    }

    fn replace_at_cursor(cursor: &mut Self::Cursor, func: impl FnOnce(&mut Self::Cursor)) {
        let mut replacement_cursor = Cursor::Detached;
        func(&mut replacement_cursor);

        match (&cursor, replacement_cursor) {
            (Cursor::Detached, _) => {}
            (Cursor::Node(node), Cursor::Node(replacement)) => {
                let parent = node.parent_element().unwrap();
                parent.replace_child(&replacement, node).unwrap();

                *cursor = Cursor::Node(replacement);
            }
            (Cursor::Node(_node), Cursor::Detached) => {
                panic!();
            }
            (Cursor::AttrsOf(_el), _) => {
                panic!()
            }
            (Cursor::EmptyChildrenOf(_), _) => {
                panic!();
            }
            _ => panic!(),
        }
    }

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(task);
    }

    fn debug_start_reactive_update(cursor: &mut Self::Cursor) {
        log(&format!("start reactive update for {cursor:?}"));
    }
}

#[derive(Clone, Debug)]
pub enum Cursor {
    Detached,
    Node(web_sys::Node),
    EmptyChildrenOf(web_sys::Element),
    AttrsOf(web_sys::Element),
}

impl Cursor {
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

#[inline]
fn dom_node(handle: &Handle) -> &web_sys::Node {
    match handle {
        Handle::DomNode(node) => node,
        _ => panic!(),
    }
}
