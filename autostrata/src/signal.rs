use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures::{SinkExt, StreamExt};

use crate::registry::{Registry, ViewId, REGISTRY};

static NEXT_SIGNAL: AtomicU64 = AtomicU64::new(0);

/// The Id of a signal
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Signal(u64);

impl Signal {
    pub(crate) fn alloc() -> Self {
        Self(NEXT_SIGNAL.fetch_add(1, Ordering::SeqCst))
    }

    /// Send the signal
    pub(crate) fn send(self) {
        REGISTRY.with_borrow_mut(|registry| {
            if registry.pending_signals.is_empty() {
                let mut sender = get_signal_sender(registry);

                #[cfg(feature = "web")]
                wasm_bindgen_futures::spawn_local(async move {
                    sender.send(()).await.unwrap();
                });
            }

            registry.pending_signals.insert(self);
        })
    }

    /// register a dependency upon a signal.
    ///
    /// This will register a subscription between the current active subscriber (if any) and the signal.
    pub(crate) fn register_reactive_dependency(self) {
        REGISTRY.with_borrow_mut(|registry| {
            if let Some(view_id) = registry.current_reactive_view {
                registry.put_subscription(self, view_id);
            }
        });
    }
}

/// A signal handler
pub trait OnSignal {
    fn on_signal(&self, target: ViewId) -> bool;
}

/// Broadcast the set of signals to all subscribers.
///
/// Each implicated subscriber will only be notified once,
/// even if it subscribes to several of the signals.
fn broadcast(signals: HashSet<Signal>) {
    let callbacks_by_view_id = REGISTRY.with_borrow(|registry| {
        let mut callbacks: HashMap<ViewId, Rc<dyn OnSignal>> = Default::default();

        for signal in signals {
            if let Some(subscriptions) = registry.subscriptions_by_signal.get(&signal) {
                for view_id in subscriptions {
                    match callbacks.entry(*view_id) {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(vacant) => {
                            vacant
                                .insert(registry.reactive_callbacks.get(view_id).unwrap().clone());
                        }
                    }
                }
            }
        }

        callbacks
    });

    for (view_id, callback) in callbacks_by_view_id {
        callback.on_signal(view_id);
    }
}

fn get_signal_sender(registry: &mut Registry) -> futures::channel::mpsc::Sender<()> {
    if let Some(sender) = &registry.signal_sender {
        sender.clone()
    } else {
        let (sender, mut receiver) = futures::channel::mpsc::channel::<()>(1);

        #[cfg(feature = "web")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                loop {
                    if let Some(()) = receiver.next().await {
                        let pending_signals = REGISTRY.with_borrow_mut(|registry| {
                            std::mem::take(&mut registry.pending_signals)
                        });

                        broadcast(pending_signals);
                    } else {
                        panic!("signal connection lost");
                    }
                }
            });
        }

        registry.signal_sender = Some(sender.clone());
        sender
    }
}