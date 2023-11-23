use std::sync::{Arc, Mutex};

use futures::SinkExt;

use crate::pubsub::new_signal_id;

pub struct State<T> {
    mutex: Arc<Mutex<Arc<T>>>,
    signal_id: u64,
}

impl<T: 'static> State<T> {
    pub fn get(&self) -> Arc<T> {
        let lock = self.mutex.lock().unwrap();
        (*lock).clone()
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            mutex: self.mutex.clone(),
            signal_id: self.signal_id,
        }
    }
}

pub struct StateMut<T> {
    mutex: Arc<Mutex<Arc<T>>>,
    tx: futures::channel::mpsc::Sender<()>,
    signal_id: u64,
}

impl<T: 'static> StateMut<T> {
    pub fn set(&self, value: T) {
        {
            let mut lock = self.mutex.lock().unwrap();
            *lock = Arc::new(value);
        }
        self.notify();
    }

    pub fn update(&self, func: impl Fn(&T) -> T) {
        {
            let mut lock = self.mutex.lock().unwrap();
            let new_value = func(&*lock);
            *lock = Arc::new(new_value);
        }
        self.notify();
    }

    fn notify(&self) {
        let mut tx = self.tx.clone();

        #[cfg(feature = "dom")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = tx.send(()).await;
            });
        }
    }
}

impl<T> Clone for StateMut<T> {
    fn clone(&self) -> Self {
        Self {
            signal_id: self.signal_id,
            mutex: self.mutex.clone(),
            tx: self.tx.clone(),
        }
    }
}

pub fn use_state<T: 'static>(value: T) -> (State<T>, StateMut<T>) {
    let signal_id = new_signal_id();
    let mutex = Arc::new(Mutex::new(Arc::new(value)));
    let (tx, rx) = futures::channel::mpsc::channel(1);

    (
        State {
            signal_id,
            mutex: mutex.clone(),
        },
        StateMut {
            signal_id,
            mutex,
            tx,
        },
    )
}
