use std::{marker::PhantomData, rc::Rc};

use crate::{pubsub::SignalId, registry::REGISTRY};

pub fn use_state<T: 'static>(value: T) -> (State<T>, StateMut<T>) {
    let signal_id = SignalId::alloc();

    REGISTRY.with_borrow_mut(|registry| {
        let view_id = registry
            .current_func_view
            .expect("There must be a Func view in scope");

        registry
            .owned_signals
            .entry(view_id)
            .or_default()
            .push(signal_id);

        registry.state_values.insert(signal_id, Rc::new(value));
    });

    (
        State {
            signal_id,
            phantom: PhantomData,
        },
        StateMut {
            signal_id,
            phantom: PhantomData,
        },
    )
}

pub struct State<T> {
    signal_id: SignalId,
    phantom: PhantomData<T>,
}

impl<T: 'static> State<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.signal_id.register_reactive_dependency();

        let rc_value = REGISTRY
            .with_borrow(|registry| registry.state_values.get(&self.signal_id).unwrap().clone());
        let value_ref = rc_value.downcast_ref::<T>().unwrap();

        value_ref.clone()
    }

    pub fn read<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.signal_id.register_reactive_dependency();
        REGISTRY.with_borrow(|registry| {
            f(registry
                .state_values
                .get(&self.signal_id)
                .unwrap()
                .downcast_ref::<T>()
                .unwrap())
        })
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            signal_id: self.signal_id,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for State<T> {}

pub struct StateMut<T> {
    signal_id: SignalId,
    phantom: PhantomData<T>,
}

impl<T: 'static> StateMut<T> {
    pub fn set(&self, value: T) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.state_values.insert(self.signal_id, Rc::new(value));
        });

        self.send();
    }

    pub fn update(&self, func: impl Fn(&T) -> T) {
        let new_value = REGISTRY.with_borrow(|registry| {
            let reference = registry
                .state_values
                .get(&self.signal_id)
                .unwrap()
                .downcast_ref::<T>()
                .unwrap();

            func(reference)
        });

        REGISTRY.with_borrow_mut(|registry| {
            registry
                .state_values
                .insert(self.signal_id, Rc::new(new_value));
        });

        self.send();
    }

    fn send(&self) {
        let signaller = REGISTRY.with_borrow_mut(|registry| registry.get_signaller());
        signaller.send(self.signal_id);
    }
}

impl<T> Clone for StateMut<T> {
    fn clone(&self) -> Self {
        Self {
            signal_id: self.signal_id,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for StateMut<T> {}
