use std::cell::RefCell;

pub trait Notify {
    fn notify(&self) -> bool;
}

thread_local! {
    static ACTIVE_NOTIFIER: RefCell<ActiveNotifier> = RefCell::new(ActiveNotifier { current_notifier: RefCell::new(None) });
}

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
