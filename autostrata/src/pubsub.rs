use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SignalId(u64);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SubscriberId(u64);

pub trait Notify {
    fn notify(&self, signal_id: SignalId, subscriber_id: SubscriberId) -> bool;
}

#[derive(Clone)]
pub struct SubscriberHandle {
    gc: Arc<SubscriberGc>,
}

impl SubscriberHandle {
    pub fn subscriber_id(&self) -> SubscriberId {
        self.gc.subscriber_id
    }
}

struct SubscriberGc {
    subscriber_id: SubscriberId,
}

impl Drop for SubscriberGc {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            let subscriber_id = self.subscriber_id;
            registry.notifiers.remove(&subscriber_id);
            registry
                .subscriptions
                .retain(|entry| entry.1 != subscriber_id);
        });
    }
}

thread_local! {
    static ACTIVE_SUBSCRIBER: RefCell<Option<SubscriberId>> = RefCell::new(None);

    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

static SIGNAL_ID: AtomicU64 = AtomicU64::new(0);

static SUBSCRIBER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct Registry {
    notifiers: HashMap<SubscriberId, Arc<dyn Notify>>,
    subscriptions: BTreeSet<(SignalId, SubscriberId)>,
}

pub(crate) fn new_subscriber(notify: Arc<dyn Notify>) -> SubscriberHandle {
    let subscriber_id = SubscriberId(SUBSCRIBER_ID.fetch_add(1, Ordering::SeqCst));

    REGISTRY.with_borrow_mut(|registry| {
        registry.notifiers.insert(subscriber_id, notify);
    });

    SubscriberHandle {
        gc: Arc::new(SubscriberGc { subscriber_id }),
    }
}

pub(crate) fn new_signal_id() -> SignalId {
    SignalId(SIGNAL_ID.fetch_add(1, Ordering::SeqCst))
}

/// Set the subscriber id as the current reactive one,
/// and execute the given function, before resetting state to previous state again.
///
/// Setting a subscriber to the current reactive one, enables
/// automatic subscription creation when a signal dependency is registered.
pub(crate) fn with_current_reactive_subscriber<T>(
    subscriber_id: SubscriberId,
    func: impl FnOnce() -> T,
) -> T {
    let prev = ACTIVE_SUBSCRIBER
        .with_borrow_mut(|active_subscriber| active_subscriber.replace(subscriber_id));

    let ret = func();

    ACTIVE_SUBSCRIBER.with_borrow_mut(|active_subscriber| {
        *active_subscriber.borrow_mut() = prev;
    });

    ret
}

/// register a dependency upon a signal.
///
/// This will register a subscription between the current active subscriber (if any) and the signal.
pub(crate) fn register_signal_dependency(signal_id: SignalId) {
    let active = ACTIVE_SUBSCRIBER.with_borrow_mut(|active| active.clone());

    if let Some(subscriber_id) = active {
        REGISTRY.with_borrow_mut(|registry| {
            registry.subscriptions.insert((signal_id, subscriber_id));
        })
    }
}

pub(crate) fn notify(signal_id: SignalId) {
    let notifiers: Vec<(Arc<dyn Notify>, SubscriberId)> = REGISTRY.with_borrow_mut(|registry| {
        // FIXME: More optimal traversal of subscription
        registry
            .subscriptions
            .iter()
            .filter_map(|(filter_signal_id, subscriber_id)| {
                if filter_signal_id == &signal_id {
                    if let Some(notify) = registry.notifiers.get(subscriber_id) {
                        Some((notify.clone(), *subscriber_id))
                    } else {
                        // FIXME: BUG
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    });

    for (notifier, subscriber_id) in notifiers {
        notifier.notify(signal_id, subscriber_id);
    }
}
