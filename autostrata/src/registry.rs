use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::pubsub::{OnSignal, SignalId};

static NEXT_REACTIVE_ID: AtomicU64 = AtomicU64::new(0);

/// The Id of a subscriber
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ReactiveId(u64);

impl ReactiveId {
    pub(crate) fn alloc() -> Self {
        Self(NEXT_REACTIVE_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// Set the reactive id as current in a reactive operation,
    /// and execute the given function, before resetting state to previous state again.
    ///
    /// Setting a reactive to the current one, enables
    /// automatic subscription creation when a signal dependency is registered.
    pub(crate) fn invoke_as_current<T>(self, func: impl FnOnce() -> T) -> T {
        let prev_id = CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|current| current.replace(self));

        let value = func();

        CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|current| {
            *current.borrow_mut() = prev_id;
        });

        value
    }
}

thread_local! {
    pub(crate) static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());

    pub(crate) static CURRENT_REACTIVE_SUBSCRIBER: RefCell<Option<ReactiveId>> = RefCell::new(None);
}

#[derive(Default)]
pub(crate) struct Registry {
    pub(crate) callbacks: HashMap<ReactiveId, Rc<dyn OnSignal>>,
    pub(crate) subscriptions_by_signal: HashMap<SignalId, BTreeSet<ReactiveId>>,
    pub(crate) subscriptions_by_subscriber: HashMap<ReactiveId, BTreeSet<SignalId>>,
    pub(crate) signal_sender: Option<futures::channel::mpsc::UnboundedSender<SignalId>>,
}

impl Registry {
    pub(crate) fn put_subscription(&mut self, signal_id: SignalId, reactive_id: ReactiveId) {
        self.subscriptions_by_signal
            .entry(signal_id)
            .or_default()
            .insert(reactive_id);
        self.subscriptions_by_subscriber
            .entry(reactive_id)
            .or_default()
            .insert(signal_id);
    }

    pub(crate) fn remove_subscriber(&mut self, reactive_id: ReactiveId) {
        self.callbacks.remove(&reactive_id);

        if let Some(signals) = self.subscriptions_by_subscriber.remove(&reactive_id) {
            for signal_id in signals {
                remove_set_entry(&mut self.subscriptions_by_signal, &signal_id, &reactive_id);
            }
        }
    }

    pub(crate) fn remove_signal(&mut self, signal_id: SignalId) {
        if let Some(subscribers) = self.subscriptions_by_signal.remove(&signal_id) {
            for reactive_id in subscribers {
                remove_set_entry(
                    &mut self.subscriptions_by_subscriber,
                    &reactive_id,
                    &signal_id,
                );
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
