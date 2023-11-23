use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

pub trait Notify {
    fn subscriber_id(&self) -> u64;
    fn notify(&self) -> bool;
}

thread_local! {
    static ACTIVE_NOTIFIER: RefCell<ActiveNotifier> = RefCell::new(ActiveNotifier { current_notifier: RefCell::new(None) });
}

static SIGNAL_ID: AtomicU64 = AtomicU64::new(0);

static SUBSCRIBER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct ActiveNotifier {
    current_notifier: RefCell<Option<Box<dyn Notify>>>,
}

pub(crate) fn with_active_notifier<T>(notifier: Box<dyn Notify>, func: impl FnOnce() -> T) -> T {
    ACTIVE_NOTIFIER.with_borrow_mut(|active_notifier| {
        let previous_notifier = active_notifier
            .current_notifier
            .borrow_mut()
            .replace(notifier);

        let ret = func();

        *active_notifier.current_notifier.borrow_mut() = previous_notifier;

        ret
    })
}

pub(crate) fn new_signal_id() -> u64 {
    SIGNAL_ID.fetch_add(1, Ordering::SeqCst)
}

pub(crate) fn new_subscriber_id() -> u64 {
    SUBSCRIBER_ID.fetch_add(1, Ordering::SeqCst)
}

/*
pub struct Signal<T> {
    phantom: PhantomData<T>,
}

pub struct SignalMut<T> {
    tx: Arc<Mutex<futures::channel::mpsc::Sender<T>>>,
}

impl<T> SignalMut<T> {
    pub async fn set(&mut self, value: T) {
        let mut lock = self.tx.lock().unwrap();
        let _ = lock.try_send(value);
    }
}

async fn signal_reader<T>(mut receiver: futures::channel::mpsc::Receiver<T>) {
    while let Some(value) = receiver.next().await {}
}

*/
