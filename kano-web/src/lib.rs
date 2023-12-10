//! Kano is a work-in-progress GUI application framework written for and in Rust.
#![allow(non_snake_case, non_upper_case_globals)]

use std::rc::Rc;

use anyhow::anyhow;
use futures::{SinkExt, StreamExt};
use kano::platform::{Platform, PlatformContext, PlatformInit};
use wasm_bindgen::prelude::*;
use web_cursor::{Position, WebCursor};
use web_sys::{window, Document};

mod diff;
mod web_cursor;

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
            history_api: Rc::new({
                let location = window().unwrap().location();
                let mut loc = location.pathname().unwrap();
                if let Ok(hash) = location.hash() {
                    loc.push_str(&hash);
                }

                kano::history::HistoryState::new(loc)
            }),
        }
    }

    fn run(view: impl kano::View<Self>, _context: PlatformContext) -> anyhow::Result<()> {
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
