use std::{cell::RefCell, rc::Rc};

use crate::pubsub::{Signal, SignalId};

pub fn use_state<T: 'static>(value: T) -> (State<T>, StateMut<T>) {
    let signal = Signal::new();
    let value = Rc::new(RefCell::new(Rc::new(value)));

    (
        State {
            signal_id: signal.id(),
            value: value.clone(),
        },
        StateMut { value, signal },
    )
}

pub struct State<T> {
    value: Rc<RefCell<Rc<T>>>,
    signal_id: SignalId,
}

impl<T: 'static> State<T> {
    pub fn get(&self) -> Rc<T> {
        self.signal_id.register_dependency();

        self.value.borrow().clone()
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            signal_id: self.signal_id,
        }
    }
}

pub struct StateMut<T> {
    value: Rc<RefCell<Rc<T>>>,
    signal: Signal,
}

impl<T: 'static> StateMut<T> {
    pub fn set(&self, value: T) {
        {
            *self.value.borrow_mut() = Rc::new(value);
        }
        self.signal.send();
    }

    pub fn update(&self, func: impl Fn(&T) -> T) {
        {
            let old = self.value.borrow();
            let new_value = func(&*old);
            drop(old);

            *self.value.borrow_mut() = Rc::new(new_value);
        }
        self.signal.send();
    }
}

impl<T> Clone for StateMut<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            signal: self.signal.clone(),
        }
    }
}
