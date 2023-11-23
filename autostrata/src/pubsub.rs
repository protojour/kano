use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use futures::{SinkExt, StreamExt};

/// The Id of a signal
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SignalId(u64);

impl SignalId {
    /// register a dependency upon a signal.
    ///
    /// This will register a subscription between the current active subscriber (if any) and the signal.
    pub(crate) fn register_dependency(self) {
        let active = CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|active| active.clone());

        if let Some(subscriber_id) = active {
            REGISTRY.with_borrow_mut(|registry| {
                registry.set_subscription(self, subscriber_id);
            })
        }
    }
}

/// The Id of a subscriber
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SubscriberId(u64);

impl SubscriberId {
    /// Set the subscriber id as current in a reactive operation,
    /// and execute the given function, before resetting state to previous state again.
    ///
    /// Setting a subscriber to the current reactive one, enables
    /// automatic subscription creation when a signal dependency is registered.
    pub(crate) fn invoke_as_current_reactive<T>(self, func: impl FnOnce() -> T) -> T {
        let prev_id = CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|current| current.replace(self));

        let value = func();

        CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|current| {
            *current.borrow_mut() = prev_id;
        });

        value
    }
}

#[derive(Clone)]
pub struct Signal {
    signal_gc: Arc<SignalGc>,
}

impl Signal {
    /// Allocate a new signal.
    ///
    /// A signal can be sent at any time to notify subscribers to it.
    pub fn new() -> Signal {
        let notification_sender = REGISTRY.with_borrow_mut(|registry| {
            if let Some(signal_sender) = &registry.signal_sender {
                signal_sender.clone()
            } else {
                let (sender, mut receiver) = futures::channel::mpsc::unbounded::<SignalId>();

                #[cfg(feature = "dom")]
                {
                    wasm_bindgen_futures::spawn_local(async move {
                        loop {
                            if let Some(signal_id) = receiver.next().await {
                                crate::log(&format!("signal received: {signal_id:?}"));
                                broadcast_signal(signal_id);
                            } else {
                                panic!("signal connection lost");
                            }
                        }
                    });
                }

                registry.signal_sender = Some(sender.clone());
                sender
            }
        });

        Signal {
            signal_gc: Arc::new(SignalGc {
                signal_id: SignalId(SIGNAL_ID.fetch_add(1, Ordering::SeqCst)),
                signal_tx: notification_sender.clone(),
            }),
        }
    }

    /// Send the signal to all current subscribers
    pub fn send(&self) {
        let signal_gc = self.signal_gc.as_ref();

        let signal_id = signal_gc.signal_id;
        let mut signal_tx = signal_gc.signal_tx.clone();

        #[cfg(feature = "dom")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = signal_tx.send(signal_id).await;
            });
        }
    }

    /// The id of the signal
    pub fn id(&self) -> SignalId {
        self.signal_gc.signal_id
    }
}

/// A signal handler
pub trait OnSignal {
    fn on_signal(&self, signal_id: SignalId, subscriber_id: SubscriberId) -> bool;
}

/// A subscriber associates a SubscriberId with an actual notification callback.
#[derive(Clone)]
pub struct Subscriber {
    subscriber_gc: Arc<SubscriberGc>,
}

impl Subscriber {
    /// Create a new subscriber that wraps the given OnSignal callback.
    ///
    /// The subscriber can be associated with any signal after creation.
    ///
    /// The relationship between the subscriber's ID and the OnSignal handler
    /// is retained while the returned Subscriber is in scope.
    pub fn new(notify: Arc<dyn OnSignal>) -> Self {
        let subscriber_id = SubscriberId(SUBSCRIBER_ID.fetch_add(1, Ordering::SeqCst));

        REGISTRY.with_borrow_mut(|registry| {
            registry.callbacks.insert(subscriber_id, notify);
        });

        Subscriber {
            subscriber_gc: Arc::new(SubscriberGc { subscriber_id }),
        }
    }

    pub fn id(&self) -> SubscriberId {
        self.subscriber_gc.subscriber_id
    }
}

struct SignalGc {
    signal_id: SignalId,
    signal_tx: futures::channel::mpsc::UnboundedSender<SignalId>,
}

impl Drop for SignalGc {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.remove_signal(self.signal_id);
        });
    }
}

struct SubscriberGc {
    subscriber_id: SubscriberId,
}

impl Drop for SubscriberGc {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.remove_subscriber(self.subscriber_id);
        });
    }
}

thread_local! {
    static CURRENT_REACTIVE_SUBSCRIBER: RefCell<Option<SubscriberId>> = RefCell::new(None);

    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

static SIGNAL_ID: AtomicU64 = AtomicU64::new(0);

static SUBSCRIBER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct Registry {
    callbacks: HashMap<SubscriberId, Arc<dyn OnSignal>>,
    subscriptions_by_signal: HashMap<SignalId, BTreeSet<SubscriberId>>,
    subscriptions_by_subscriber: HashMap<SubscriberId, BTreeSet<SignalId>>,
    signal_sender: Option<futures::channel::mpsc::UnboundedSender<SignalId>>,
}

impl Registry {
    fn set_subscription(&mut self, signal_id: SignalId, subscriber_id: SubscriberId) {
        self.subscriptions_by_signal
            .entry(signal_id)
            .or_default()
            .insert(subscriber_id);
        self.subscriptions_by_subscriber
            .entry(subscriber_id)
            .or_default()
            .insert(signal_id);
    }

    fn remove_subscriber(&mut self, subscriber_id: SubscriberId) {
        self.callbacks.remove(&subscriber_id);

        if let Some(signals) = self.subscriptions_by_subscriber.remove(&subscriber_id) {
            for signal_id in signals {
                remove_set_entry(
                    &mut self.subscriptions_by_signal,
                    &signal_id,
                    &subscriber_id,
                );
            }
        }
    }

    fn remove_signal(&mut self, signal_id: SignalId) {
        if let Some(subscribers) = self.subscriptions_by_signal.remove(&signal_id) {
            for subscriber_id in subscribers {
                remove_set_entry(
                    &mut self.subscriptions_by_subscriber,
                    &subscriber_id,
                    &signal_id,
                );
            }
        }
    }
}

/// Send the given signal to all subscribers
fn broadcast_signal(signal_id: SignalId) {
    // Don't invoke callbacks while holding the registry lock.
    // Collect into a vector first.
    let callbacks: Vec<(Arc<dyn OnSignal>, SubscriberId)> = REGISTRY.with_borrow_mut(|registry| {
        registry
            .subscriptions_by_signal
            .get(&signal_id)
            .into_iter()
            .flat_map(|subscribers| {
                subscribers.iter().map(|subscriber_id| {
                    (
                        registry.callbacks.get(subscriber_id).unwrap().clone(),
                        *subscriber_id,
                    )
                })
            })
            .collect()
    });

    for (callback, subscriber_id) in callbacks {
        callback.on_signal(signal_id, subscriber_id);
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
