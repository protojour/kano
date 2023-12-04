#![allow(dead_code, unused_imports, unused_variables)]

use crate::{registry::REGISTRY, signal::Signal};

pub fn current_location() -> String {
    #[cfg(feature = "routing")]
    {
        let (signal, current) = REGISTRY.with_borrow(|registry| {
            let history = &registry.globals.history;
            (history.signal, history.entries.last().unwrap().clone())
        });

        signal.register_reactive_dependency();

        current
    }

    #[cfg(not(feature = "routing"))]
    {
        String::new()
    }
}

pub fn push(location: String) {
    #[cfg(feature = "routing")]
    {
        let signal = REGISTRY.with_borrow_mut(move |registry| {
            registry.globals.history.entries.push(location);
            registry.globals.history.signal
        });
        signal.send();
    }
}

pub fn pop() -> bool {
    #[cfg(feature = "routing")]
    {
        let (signal, result) = REGISTRY.with_borrow_mut(move |registry| {
            let result = if registry.globals.history.entries.len() == 1 {
                false
            } else {
                registry.globals.history.entries.pop();
                true
            };
            (registry.globals.history.signal, result)
        });
        signal.send();
        return result;
    }

    #[allow(unreachable_code)]
    false
}

pub(crate) struct History {
    signal: Signal,
    entries: Vec<String>,
}

impl History {
    pub fn new(signal: Signal) -> Self {
        Self {
            signal,
            entries: vec![],
        }
    }
}
