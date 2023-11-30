//! Kano is a work-in-progress GUI application framework written for and in Rust.
#![allow(non_snake_case, non_upper_case_globals)]

use std::rc::Rc;

use anyhow::anyhow;
use futures::{SinkExt, StreamExt};
use gloo::events::EventListener;
use js_sys::wasm_bindgen::*;
use kano::platform::{Platform, PlatformContext};
use wasm_bindgen::prelude::*;
use web_sys::{window, Document};
use web_sys::{Element, EventTarget};

use kano::{Event, On};

pub mod html;
pub mod html_attrs;

mod element;
mod props;

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

    fn init(signal_dispatch: Box<dyn Fn()>) -> PlatformContext {
        console_error_panic_hook::set_once();

        let (dispatch_tx, mut dispatch_rx) = futures::channel::mpsc::channel::<()>(1);
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                if let Some(()) = dispatch_rx.next().await {
                    signal_dispatch();
                } else {
                    panic!("signal connection lost");
                }
            }
        });

        PlatformContext {
            on_signal_tick: Rc::new(move || {
                let mut tx = dispatch_tx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    tx.send(()).await.unwrap();
                });
            }),
            signal_dispatch: Box::new(|| {}),
            logger: Rc::new(|s| {
                js::log(s);
            }),
        }
    }

    fn run(view: impl kano::View<Self>, _context: PlatformContext) -> anyhow::Result<()> {
        let mut cursor = WebCursor::Detached;
        let state = view.init(&mut cursor);

        let WebCursor::Node(node, _) = cursor else {
            return Err(anyhow!("No node rendered"));
        };

        document()
            .body()
            .unwrap()
            .append_child(&node)
            .map_err(|e| anyhow!("{e:?}"))?;

        // Need to keep the initial state around, it keeps EventListeners alive
        std::mem::forget(state);
        Ok(())
    }

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(task);
    }
}

#[derive(Clone, Debug)]
pub enum WebCursor {
    Detached,
    Node(web_sys::Node, Mode),
    AfterLastChild(web_sys::Element, Mode),
}

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Append,
    Diff,
}

impl WebCursor {
    fn mode(&self) -> Mode {
        match self {
            WebCursor::AfterLastChild(_, mode) => *mode,
            WebCursor::Node(_, mode) => *mode,
            WebCursor::Detached => Mode::Append,
        }
    }
}

impl WebCursor {
    fn element(&mut self, tag: &str) -> web_sys::Node {
        match self.mode() {
            Mode::Append => {
                let element = document().create_element(tag).unwrap();
                self.append_node(&element);
                // log(&format!("new element cursor: {cursor:?}"));
                element.into()
            }
            Mode::Diff => match self {
                Self::Node(..) => self.handle(),
                _ => panic!(),
            },
        }
    }

    fn on_event(&mut self, on_event: On<Event>) -> EventListener {
        match self {
            WebCursor::Node(element, _mode) => {
                kano::log("on_event");
                let event_target: &EventTarget = element.dyn_ref().unwrap();
                let event_type = match on_event.event() {
                    Event::Click => "click",
                    Event::MouseOver => "mouseover",
                };

                EventListener::new(event_target, event_type, move |_| {
                    on_event.invoke();
                })
            }
            WebCursor::Detached => panic!(),
            WebCursor::AfterLastChild(..) => panic!(),
        }
    }

    pub fn get_element(&self) -> &web_sys::Element {
        match self {
            WebCursor::Node(node, _mode) => node.dyn_ref().unwrap(),
            _ => panic!(),
        }
    }
}

impl kano::platform::Cursor for WebCursor {
    type TextHandle = web_sys::Node;
    type EventHandle = gloo::events::EventListener;

    fn from_text_handle(handle: &web_sys::Node) -> Self {
        Self::Node(handle.clone(), Mode::Append)
    }

    fn empty(&mut self) {
        match &self {
            _ => {
                let comment = document().create_comment("");
                self.append_node(&comment);
            }
        }
    }

    fn text(&mut self, text: &str) -> web_sys::Node {
        match self.mode() {
            Mode::Append => {
                let text_node = document().create_text_node(text);
                self.append_node(&text_node);
                text_node.into()
            }
            Mode::Diff => {
                self.update_text(text);
                self.handle()
            }
        }
    }

    fn update_text(&mut self, text: &str) {
        match self {
            Self::Node(node, _) => {
                node.set_node_value(Some(text));
            }
            _ => panic!(),
        }
    }

    fn enter_children(&mut self) {
        match self {
            WebCursor::Node(node, mode) => {
                if let Some(child) = node.first_child() {
                    // log(&format!("enter child: had child {child:?}"));
                    *self = WebCursor::Node(child, *mode);
                } else if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                    // log(&format!("enter child: had no children"));
                    *self = WebCursor::AfterLastChild(element.clone(), *mode);
                } else {
                    panic!("No children");
                }
            }
            WebCursor::AfterLastChild(_, _) | WebCursor::Detached => {
                panic!("Enter empty children");
            }
        }
    }

    fn exit_children(&mut self) {
        // log("exit child");
        match self {
            WebCursor::Node(node, mode) => {
                let parent = node.parent_element().unwrap();
                *self = WebCursor::Node(parent.dyn_into().unwrap(), *mode);
            }
            WebCursor::AfterLastChild(element, mode) => {
                *self = WebCursor::Node(element.dyn_ref::<web_sys::Node>().unwrap().clone(), *mode);
            }
            WebCursor::Detached => panic!("no children"),
        }
    }

    fn next_sibling(&mut self) {
        match &self {
            WebCursor::AfterLastChild(..) => {}
            WebCursor::Detached => panic!(),
            WebCursor::Node(node, mode) => {
                if let Some(next) = node.next_sibling() {
                    *self = WebCursor::Node(next, *mode);
                } else {
                    let parent = node.parent_element().unwrap();
                    *self = WebCursor::AfterLastChild(parent, *mode);
                }
            }
        }
    }

    fn remove(&mut self) {
        match &self {
            WebCursor::AfterLastChild(..) => panic!(),
            WebCursor::Detached => panic!(),
            WebCursor::Node(node, mode) => {
                let next = if let Some(next) = node.next_sibling() {
                    WebCursor::Node(next, *mode)
                } else {
                    let parent = node.parent_element().unwrap();
                    WebCursor::AfterLastChild(parent, *mode)
                };

                if let Some(element) = node.dyn_ref::<Element>() {
                    element.remove();
                } else {
                    let parent = node.parent_element().unwrap();
                    parent.remove_child(node).unwrap();
                }

                *self = next;
            }
        }
    }

    fn enter_diff(&mut self) {
        match self {
            WebCursor::AfterLastChild(_, mode) => {
                *mode = Mode::Diff;
            }
            WebCursor::Node(_, mode) => {
                *mode = Mode::Diff;
            }
            WebCursor::Detached => {}
        }
    }

    fn exit_diff(&mut self) {
        match self {
            WebCursor::AfterLastChild(_, mode) => {
                *mode = Mode::Append;
            }
            WebCursor::Node(_, mode) => {
                *mode = Mode::Append;
            }
            WebCursor::Detached => {}
        }
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let mut replacement_cursor = WebCursor::Detached;
        func(&mut replacement_cursor);

        match (&self, replacement_cursor) {
            (WebCursor::Detached, _) => {}
            (WebCursor::Node(node, _mode), WebCursor::Node(replacement, mode2)) => {
                let parent = node.parent_element().unwrap();
                parent.replace_child(&replacement, node).unwrap();

                *self = WebCursor::Node(replacement, mode2);
            }
            (WebCursor::Node(_node, _), WebCursor::Detached) => {
                panic!();
            }
            (WebCursor::AfterLastChild(..), _) => {
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
                *self = Self::Node(appendee.clone(), Mode::Append);
            }
            Self::AfterLastChild(element, mode) => {
                // log("append at empty");
                element.append_child(appendee).expect("A");
                *self = Self::Node(appendee.clone(), *mode);
            }
            Self::Node(node, mode) => {
                // log("append after node");
                node.parent_element()
                    .expect("parent element of node")
                    .insert_before(appendee, node.next_sibling().as_ref())
                    .expect("insert_before");
                *self = Self::Node(appendee.clone(), *mode);
            }
        }
    }

    fn handle(&self) -> web_sys::Node {
        match self {
            Self::Detached => panic!(),
            Self::Node(node, _) => node.clone(),
            Self::AfterLastChild(..) => panic!(),
        }
    }
}

fn document() -> Document {
    window().unwrap().document().unwrap()
}
