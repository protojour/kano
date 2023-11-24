use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use futures::{SinkExt, StreamExt};

use crate::registry::{ReactiveId, CURRENT_REACTIVE_SUBSCRIBER, REGISTRY};

static NEXT_SIGNAL_ID: AtomicU64 = AtomicU64::new(0);

/// The Id of a signal
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SignalId(u64);

impl SignalId {
    pub(crate) fn alloc() -> Self {
        Self(NEXT_SIGNAL_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// register a dependency upon a signal.
    ///
    /// This will register a subscription between the current active subscriber (if any) and the signal.
    pub(crate) fn register_dependency(self) {
        let active = CURRENT_REACTIVE_SUBSCRIBER.with_borrow_mut(|active| active.clone());

        if let Some(reactive_id) = active {
            REGISTRY.with_borrow_mut(|registry| {
                registry.put_subscription(self, reactive_id);
            })
        }
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

                #[cfg(feature = "web")]
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
                signal_id: SignalId::alloc(),
                signal_tx: notification_sender.clone(),
            }),
        }
    }

    /// Send the signal to all current subscribers
    pub fn send(&self) {
        let signal_gc = self.signal_gc.as_ref();

        let signal_id = signal_gc.signal_id;
        let mut signal_tx = signal_gc.signal_tx.clone();

        #[cfg(feature = "web")]
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
    fn on_signal(&self, id: SignalId, target: ReactiveId) -> bool;
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
    pub fn new(reactive_id: ReactiveId, notify: Rc<dyn OnSignal>) -> Self {
        REGISTRY.with_borrow_mut(|registry| {
            registry.callbacks.insert(reactive_id, notify);
        });

        Subscriber {
            subscriber_gc: Arc::new(SubscriberGc { reactive_id }),
        }
    }

    pub fn id(&self) -> ReactiveId {
        self.subscriber_gc.reactive_id
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
    reactive_id: ReactiveId,
}

impl Drop for SubscriberGc {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.remove_subscriber(self.reactive_id);
        });
    }
}

/// Send the given signal to all subscribers
fn broadcast_signal(signal_id: SignalId) {
    // Don't invoke callbacks while holding the registry lock.
    // Collect into a vector first.
    let callbacks: Vec<(Rc<dyn OnSignal>, ReactiveId)> = REGISTRY.with_borrow_mut(|registry| {
        registry
            .subscriptions_by_signal
            .get(&signal_id)
            .into_iter()
            .flat_map(|subscribers| {
                subscribers.iter().map(|reactive_id| {
                    (
                        registry.callbacks.get(reactive_id).unwrap().clone(),
                        *reactive_id,
                    )
                })
            })
            .collect()
    });

    for (callback, target_id) in callbacks {
        callback.on_signal(signal_id, target_id);
    }
}
