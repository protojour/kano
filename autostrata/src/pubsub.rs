use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use futures::channel::mpsc::UnboundedSender;
use futures::{SinkExt, StreamExt};

use crate::registry::{Registry, ViewId, REGISTRY};

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
    pub(crate) fn register_reactive_dependency(self) {
        REGISTRY.with_borrow_mut(|registry| {
            if let Some(view_id) = registry.current_reactive_view {
                registry.put_subscription(self, view_id);
            }
        });
    }
}

#[derive(Clone)]
pub struct Signaller(UnboundedSender<SignalId>);

impl Signaller {
    /// Send the signal to all current subscribers
    pub fn send(&self, signal_id: SignalId) {
        let mut sender = self.0.clone();

        #[cfg(feature = "web")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = sender.send(signal_id).await;
            });
        }
    }
}

/// A signal handler
pub trait OnSignal {
    fn on_signal(&self, id: SignalId, target: ViewId) -> bool;
}

/// Send the given signal to all subscribers
fn broadcast_signal(signal_id: SignalId) {
    // Don't invoke callbacks while holding the registry lock.
    // Collect into a vector first.
    let callbacks: Vec<(Rc<dyn OnSignal>, ViewId)> = REGISTRY.with_borrow_mut(|registry| {
        registry
            .subscriptions_by_signal
            .get(&signal_id)
            .into_iter()
            .flat_map(|subscribers| {
                subscribers.iter().map(|view_id| {
                    (
                        registry.reactive_callbacks.get(view_id).unwrap().clone(),
                        *view_id,
                    )
                })
            })
            .collect()
    });

    for (callback, target_id) in callbacks {
        callback.on_signal(signal_id, target_id);
    }
}

impl Registry {
    pub(crate) fn get_signaller(&mut self) -> Signaller {
        if let Some(signaller) = &self.signaller {
            signaller.clone()
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

            self.signaller = Some(Signaller(sender.clone()));
            Signaller(sender)
        }
    }
}
