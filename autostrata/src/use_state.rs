use std::sync::{Arc, Mutex};

use crate::pubsub::{register_signal_dependency, Signal, SignalId};

pub fn use_state<T: 'static>(value: T) -> (State<T>, StateMut<T>) {
    let signal = Signal::new();
    let mutex = Arc::new(Mutex::new(Arc::new(value)));

    (
        State {
            signal_id: signal.signal_id(),
            mutex: mutex.clone(),
        },
        StateMut { mutex, signal },
    )
}

pub struct State<T> {
    mutex: Arc<Mutex<Arc<T>>>,
    signal_id: SignalId,
}

impl<T: 'static> State<T> {
    pub fn get(&self) -> Arc<T> {
        register_signal_dependency(self.signal_id);

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
    signal: Signal,
}

impl<T: 'static> StateMut<T> {
    pub fn set(&self, value: T) {
        {
            let mut lock = self.mutex.lock().unwrap();
            *lock = Arc::new(value);
        }
        self.signal.send();
    }

    pub fn update(&self, func: impl Fn(&T) -> T) {
        {
            let mut lock = self.mutex.lock().unwrap();
            let new_value = func(&*lock);
            *lock = Arc::new(new_value);
        }
        self.signal.send();
    }
}

impl<T> Clone for StateMut<T> {
    fn clone(&self) -> Self {
        Self {
            mutex: self.mutex.clone(),
            signal: self.signal.clone(),
        }
    }
}
