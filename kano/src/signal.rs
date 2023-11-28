use std::collections::{BTreeSet, HashMap, HashSet};

use futures::{SinkExt, StreamExt};

use crate::registry::{Registry, ViewCallback, REGISTRY};
use crate::view_id::ViewId;

/// The Id of a signal
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Signal(pub(crate) u64);

impl Signal {
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

/// Broadcast the set of signals to all subscribers.
///
/// Each implicated subscriber will only be notified once,
/// even if it subscribes to several of the signals.
fn broadcast(signals: HashSet<Signal>) {
    let callbacks_by_view_id = REGISTRY.with_borrow(|registry| {
        let view_id_set: BTreeSet<ViewId> = signals
            .iter()
            .flat_map(|signal| registry.subscriptions_by_signal.get(signal))
            .flat_map(|subscriptions| subscriptions.iter().cloned())
            .collect();

        let mut callbacks: HashMap<ViewId, ViewCallback> = Default::default();

        // The view_id_set is sorted, iterate over this _backwards_.
        // Reactive nodes deeper in the tree will be traversed before parents,
        // because children are always younger (i.e. have larger ViewId) than parents.
        for view_id in view_id_set.iter().rev() {
            let reactive_entry = registry.reactive_entries.get(view_id).unwrap();

            if let Some(reactive_parent) = reactive_entry.reactive_parent {
                // If the reactive parent is already in the view set,
                // don't register the child callback: Updating a parent
                // will implicitly update the child.
                // This process will repeat itself for parents higher up.
                if view_id_set.contains(&reactive_parent) {
                    continue;
                }
            }

            callbacks.insert(*view_id, reactive_entry.callback.clone());
        }

        callbacks
    });

    for (view_id, callback) in callbacks_by_view_id {
        callback(view_id);
    }
}

pub fn dispatch_pending_signals() {
    let pending_signals =
        REGISTRY.with_borrow_mut(|registry| std::mem::take(&mut registry.pending_signals));

    broadcast(pending_signals);
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
                        dispatch_pending_signals();
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    struct BroadcastTester {
        notified_views: Rc<RefCell<BTreeSet<ViewId>>>,
        callback: ViewCallback,
    }

    impl BroadcastTester {
        fn new() -> Self {
            let notified_views: Rc<RefCell<BTreeSet<ViewId>>> =
                Rc::new(RefCell::new(Default::default()));

            Self {
                notified_views: notified_views.clone(),
                callback: Rc::new(move |view_id| {
                    notified_views.borrow_mut().insert(view_id);
                    true
                }),
            }
        }

        fn add_reactive_view(&self) -> ViewId {
            REGISTRY.with_borrow_mut(|registry| {
                let view_id = registry.alloc_view_id();
                registry.add_reactive_view(view_id, self.callback.clone());
                view_id
            })
        }
    }

    #[test]
    fn broadcast_parent_child_deduplication() {
        REGISTRY.with_borrow_mut(Registry::reset);

        let tester = BroadcastTester::new();
        let parent0 = tester.add_reactive_view();
        let child0 = parent0.as_current_reactive(|| tester.add_reactive_view());
        let parent1 = tester.add_reactive_view();

        let signals = [Signal(0), Signal(1), Signal(2)];

        parent0.as_current_reactive(|| {
            signals[0].register_reactive_dependency();

            child0.as_current_reactive(|| {
                signals[1].register_reactive_dependency();
            })
        });
        parent1.as_current_reactive(|| signals[2].register_reactive_dependency());

        broadcast(HashSet::from(signals));

        assert_eq!(
            &*tester.notified_views.borrow(),
            &BTreeSet::from([parent0, parent1]),
            "Only the parents should be notified"
        );
    }
}
