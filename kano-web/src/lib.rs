//! Kano is a work-in-progress GUI application framework written for and in Rust.
#![allow(non_snake_case, non_upper_case_globals)]

use std::rc::Rc;

use anyhow::anyhow;
use futures::{SinkExt, StreamExt};
use gloo::events::EventListener;
use kano::{
    markup::Markup,
    platform::{Platform, PlatformContext, PlatformInit},
};
use kano_svg::Svg1_1;
use wasm_bindgen::prelude::*;
use web_cursor::{Position, WebCursor};
use web_sys::{window, Document};

mod diff;
mod web_cursor;

#[cfg(feature = "web-component")]
pub mod web_component;

pub use kano_html::Html5;

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

impl Markup<Web> for Html5 {
    type Cursor = WebCursor;
}

impl Markup<Web> for Svg1_1 {
    type Cursor = WebCursor;
}

impl Platform for Web {
    type Markup = Html5;

    fn init(init: PlatformInit) -> PlatformContext {
        console_error_panic_hook::set_once();

        let (dispatch_tx, mut dispatch_rx) = futures::channel::mpsc::channel::<()>(1);
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                if let Some(()) = dispatch_rx.next().await {
                    (init.signal_dispatch)();
                } else {
                    panic!("signal connection lost");
                }
            }
        });

        let history_refresh = init.history_refresh;

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
            history_api: Rc::new(WebHistory {
                popstate_listener: EventListener::new(&window().unwrap(), "popstate", move |_| {
                    history_refresh();
                }),
            }),
        }
    }

    fn run(view: impl kano::View<Self, Html5>, _context: PlatformContext) -> anyhow::Result<()> {
        let mut cursor = WebCursor::new_detached();
        let state = view.init(&mut cursor);

        let Position::Node(node) = cursor.position else {
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

fn document() -> Document {
    window().unwrap().document().unwrap()
}

struct WebHistory {
    #[allow(dead_code)]
    popstate_listener: EventListener,
}

impl kano::history::HistoryAPI for WebHistory {
    fn current(&self) -> String {
        #[cfg(feature = "routing")]
        {
            let location = window().unwrap().location();
            location.pathname().unwrap()
        }

        #[cfg(not(feature = "routing"))]
        String::new()
    }

    #[allow(unused_variables)]
    fn push(&self, location: String) {
        #[cfg(feature = "routing")]
        {
            let history = window().unwrap().history().unwrap();
            history
                .push_state_with_url(&JsValue::null(), "", Some(&location))
                .unwrap();
        }
    }

    fn pop(&self) -> bool {
        #[cfg(feature = "routing")]
        {
            let history = window().unwrap().history().unwrap();
            history.back().unwrap();
            true
        }

        #[cfg(not(feature = "routing"))]
        false
    }
}
