#![allow(dead_code, unused_imports, unused_variables)]

use std::{cell::RefCell, rc::Rc};

use crate::{registry::REGISTRY, signal::Signal};

pub trait HistoryAPI {
    fn current(&self) -> String;
    fn push(&self, location: String);
    fn pop(&self) -> bool;
}

pub fn current_location() -> String {
    #[cfg(feature = "routing")]
    {
        let (signal, history_api) = get();
        signal.register_reactive_dependency();
        history_api.current()
    }

    #[cfg(not(feature = "routing"))]
    {
        String::new()
    }
}

pub fn push(location: String) {
    #[cfg(feature = "routing")]
    {
        let (signal, history_api) = get();
        history_api.push(location);
        signal.send();
    }
}

pub fn pop() -> bool {
    #[cfg(feature = "routing")]
    {
        let (signal, history_api) = get();
        let result = history_api.pop();
        signal.send();
        return result;
    }

    #[allow(unreachable_code)]
    false
}

fn get() -> (Signal, Rc<dyn HistoryAPI>) {
    REGISTRY.with_borrow(move |registry| {
        (
            registry.globals.history_signal,
            registry.globals.history_api.clone(),
        )
    })
}

pub(crate) struct History {
    signal: Signal,
    entries: RefCell<Vec<String>>,
}

impl History {
    pub fn new(signal: Signal) -> Self {
        Self {
            signal,
            entries: RefCell::new(vec![]),
        }
    }
}

pub struct HistoryState {
    entries: RefCell<Vec<String>>,
}

impl HistoryState {
    pub fn new(initial_entry: String) -> Self {
        Self {
            entries: RefCell::new(vec![initial_entry]),
        }
    }
}

impl HistoryAPI for HistoryState {
    fn current(&self) -> String {
        self.entries
            .borrow()
            .last()
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }

    fn push(&self, location: String) {
        self.entries.borrow_mut().push(location);
    }

    fn pop(&self) -> bool {
        let mut entries = self.entries.borrow_mut();
        if entries.len() > 1 {
            entries.pop();
            true
        } else {
            false
        }
    }
}
