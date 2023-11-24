use std::any::Any;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::pubsub::{OnSignal, SignalId, Signaller};

static NEXT_VIEW_ID: AtomicU64 = AtomicU64::new(0);

/// A ViewId is assigned to views that do "smart things",
///
/// This includes Reactive views and other views that involves user-defined functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ViewId(u64);

impl ViewId {
    pub(crate) fn alloc() -> Self {
        Self(NEXT_VIEW_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// Set the reactive id as current in a reactive operation,
    /// and execute the given function, before resetting state to previous state again.
    ///
    /// Setting a reactive to the current one, enables
    /// automatic subscription creation when a signal dependency is registered.
    pub(crate) fn invoke_as_current_reactive_view<T>(self, func: impl FnOnce() -> T) -> T {
        let (prev_reactive, prev_func) = REGISTRY.with_borrow_mut(|registry| {
            // FIXME(?): Not backing up the signal position tracker.
            // The hypothesis is that the view should not continue to create signals
            // after it has drawn its children.
            // FIXME: Make runtime assertion for this invariant
            registry.current_func_view_signal_tracker = 0;

            (
                registry.current_reactive_view.replace(self),
                registry.current_func_view.replace(self),
            )
        });

        let value = func();

        REGISTRY.with_borrow_mut(|registry| {
            registry.current_reactive_view = prev_reactive;
            registry.current_func_view = prev_func;
        });

        value
    }

    pub(crate) fn invoke_as_current_func_view<T>(self, func: impl FnOnce() -> T) -> T {
        let prev_func = REGISTRY.with_borrow_mut(|registry| {
            // FIXME(?): Not backing up the signal position tracker.
            registry.current_func_view_signal_tracker = 0;

            registry.current_func_view.replace(self)
        });

        let value = func();

        REGISTRY.with_borrow_mut(|registry| {
            registry.current_func_view = prev_func;
        });

        value
    }
}

#[derive(Default)]
pub(crate) struct Registry {
    pub(crate) signaller: Option<Signaller>,

    pub(crate) reactive_callbacks: HashMap<ViewId, Rc<dyn OnSignal>>,
    pub(crate) subscriptions_by_signal: HashMap<SignalId, BTreeSet<ViewId>>,
    pub(crate) subscriptions_by_view: HashMap<ViewId, BTreeSet<SignalId>>,

    pub(crate) current_reactive_view: Option<ViewId>,
    pub(crate) current_func_view: Option<ViewId>,
    pub(crate) current_func_view_signal_tracker: usize,

    pub(crate) owned_signals_ordered: HashMap<ViewId, Vec<SignalId>>,
    pub(crate) state_values: HashMap<SignalId, Rc<dyn Any>>,
}

thread_local! {
    pub(crate) static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

impl Registry {
    /// Returns true if reused
    pub(crate) fn alloc_or_reuse_func_view_signal(&mut self) -> (SignalId, bool) {
        let view_id = self
            .current_func_view
            .expect("There must be a Func view in scope");

        let owned_signals_ordered = self.owned_signals_ordered.entry(view_id).or_default();

        let ret = if self.current_func_view_signal_tracker < owned_signals_ordered.len() {
            crate::log("Reusing signal");

            (
                owned_signals_ordered[self.current_func_view_signal_tracker],
                true,
            )
        } else {
            let signal_id = SignalId::alloc();
            owned_signals_ordered.push(signal_id);
            (signal_id, false)
        };

        // Track the position
        self.current_func_view_signal_tracker += 1;

        ret
    }

    pub(crate) fn put_subscription(&mut self, signal_id: SignalId, view_id: ViewId) {
        self.subscriptions_by_signal
            .entry(signal_id)
            .or_default()
            .insert(view_id);
        self.subscriptions_by_view
            .entry(view_id)
            .or_default()
            .insert(signal_id);
    }

    pub(crate) fn on_view_dropped(&mut self, view_id: ViewId) {
        if let Some(owned_signals) = self.owned_signals_ordered.remove(&view_id) {
            for signal_id in owned_signals {
                self.on_signal_dropped(signal_id);
            }
        }
    }

    pub(crate) fn on_reactive_dropped(&mut self, view_id: ViewId) {
        self.reactive_callbacks.remove(&view_id);

        if let Some(signals) = self.subscriptions_by_view.remove(&view_id) {
            for signal_id in signals {
                remove_set_entry(&mut self.subscriptions_by_signal, &signal_id, &view_id);
            }
        }
    }

    pub(crate) fn on_signal_dropped(&mut self, signal_id: SignalId) {
        self.state_values.remove(&signal_id);

        if let Some(subscribers) = self.subscriptions_by_signal.remove(&signal_id) {
            for view_id in subscribers {
                remove_set_entry(&mut self.subscriptions_by_view, &view_id, &signal_id);
            }
        }
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
