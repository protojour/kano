use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::AddAssign;
use std::rc::Rc;

use crate::signal::Signal;
use crate::view_id::ViewId;

pub type ViewCallback = Rc<dyn Fn(ViewId) -> bool>;

#[derive(Default)]
pub(crate) struct Registry {
    next_view_id: u64,
    next_signal_id: u64,

    pub signal_sender: Option<futures::channel::mpsc::Sender<()>>,
    pub pending_signals: HashSet<Signal>,

    pub subscriptions_by_signal: HashMap<Signal, BTreeSet<ViewId>>,
    pub subscriptions_by_view: HashMap<ViewId, BTreeSet<Signal>>,

    pub current_reactive_view: Option<ViewId>,
    pub current_func_view: Option<ViewId>,
    pub current_func_view_signal_tracker: usize,

    pub reactive_entries: HashMap<ViewId, ReactiveEntry>,
    pub owned_signals_ordered: HashMap<ViewId, Vec<Signal>>,
    pub state_values: HashMap<Signal, Rc<RefCell<dyn Any>>>,
}

pub(crate) struct ReactiveEntry {
    pub reactive_parent: Option<ViewId>,
    pub callback: ViewCallback,
}

thread_local! {
    pub(crate) static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

impl Registry {
    pub fn alloc_view_id(&mut self) -> ViewId {
        ViewId(fetch_add(&mut self.next_view_id, 1))
    }

    /// Returns true if reused
    pub fn alloc_or_reuse_func_view_signal(&mut self) -> (Signal, bool) {
        let view_id = self
            .current_func_view
            .expect("state should not be used outside the view hierarchy!");

        let owned_signals_ordered = self.owned_signals_ordered.entry(view_id).or_default();

        let ret = if self.current_func_view_signal_tracker < owned_signals_ordered.len() {
            crate::log("Reusing signal");

            (
                owned_signals_ordered[self.current_func_view_signal_tracker],
                true,
            )
        } else {
            let new_signal = Signal(fetch_add(&mut self.next_signal_id, 1));
            owned_signals_ordered.push(new_signal);
            (new_signal, false)
        };

        // Track the position
        self.current_func_view_signal_tracker += 1;

        ret
    }

    pub fn put_subscription(&mut self, signal: Signal, view_id: ViewId) {
        self.subscriptions_by_signal
            .entry(signal)
            .or_default()
            .insert(view_id);
        self.subscriptions_by_view
            .entry(view_id)
            .or_default()
            .insert(signal);
    }

    pub fn add_reactive_view(&mut self, view_id: ViewId, callback: ViewCallback) {
        let reactive_parent = self.current_reactive_view;

        if let Some(reactive_parent) = reactive_parent {
            debug_assert!(view_id > reactive_parent);
        }

        self.reactive_entries.insert(
            view_id,
            ReactiveEntry {
                reactive_parent,
                callback,
            },
        );
    }

    pub fn on_view_dropped(&mut self, view_id: ViewId) {
        if let Some(owned_signals) = self.owned_signals_ordered.remove(&view_id) {
            for signal in owned_signals {
                self.on_signal_dropped(signal);
            }
        }
    }

    pub fn on_reactive_dropped(&mut self, view_id: ViewId) {
        self.reactive_entries.remove(&view_id);

        if let Some(signals) = self.subscriptions_by_view.remove(&view_id) {
            for signal in signals {
                remove_set_entry(&mut self.subscriptions_by_signal, &signal, &view_id);
            }
        }
    }

    pub fn on_signal_dropped(&mut self, signal: Signal) {
        self.state_values.remove(&signal);

        if let Some(subscribers) = self.subscriptions_by_signal.remove(&signal) {
            for view_id in subscribers {
                remove_set_entry(&mut self.subscriptions_by_view, &view_id, &signal);
            }
        }
    }
}

#[cfg(test)]
impl Registry {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn peek_next_signal_id(&self) -> u64 {
        self.next_signal_id
    }
}

fn remove_set_entry<K, V>(from_map: &mut HashMap<K, BTreeSet<V>>, key: &K, value: &V)
where
    K: Eq + core::hash::Hash,
    V: Ord,
{
    if let Some(entry_set) = from_map.get_mut(key) {
        entry_set.remove(value);
        if entry_set.is_empty() {
            from_map.remove(key);
        }
    }
}

fn fetch_add<T: Copy + AddAssign<T>>(value: &mut T, add: T) -> T {
    let fetched = *value;
    *value += add;
    fetched
}
