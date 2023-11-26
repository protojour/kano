use std::{cell::RefCell, fmt::Display, marker::PhantomData, ops::Deref, rc::Rc};

use crate::{pubsub::SignalId, registry::REGISTRY};

pub fn use_state<T: 'static>(value: T) -> (State<T>, StateMut<T>) {
    let signal_id = REGISTRY.with_borrow_mut(|registry| {
        let (signal_id, reused) = registry.alloc_or_reuse_func_view_signal();

        // If the signal is reused, the value should already be in the registry,
        // and we should not reset the state.
        if !reused {
            registry
                .state_values
                .insert(signal_id, Rc::new(RefCell::new(value)));
        }

        signal_id
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

        let ref_cell = REGISTRY
            .with_borrow(|registry| registry.state_values.get(&self.signal_id).unwrap().clone());
        let borrow = ref_cell.borrow();
        let value_ref = borrow.downcast_ref::<T>().unwrap();

        value_ref.clone()
    }

    pub fn get_ref(&self) -> Ref<T> {
        let ref_cell = REGISTRY
            .with_borrow(|registry| registry.state_values.get(&self.signal_id).unwrap().clone());
        Ref {
            ref_cell,
            phantom: PhantomData,
        }
    }

    pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.signal_id.register_reactive_dependency();

        REGISTRY.with_borrow(|registry| {
            f(registry
                .state_values
                .get(&self.signal_id)
                .unwrap()
                .borrow()
                .downcast_ref::<T>()
                .unwrap())
        })
    }
}

pub struct Ref<T> {
    ref_cell: Rc<RefCell<dyn std::any::Any>>,
    phantom: PhantomData<T>,
}

impl<T: 'static> Ref<T> {
    pub fn borrow(&self) -> RefBorrow<'_, T> {
        RefBorrow {
            cell_ref: self.ref_cell.borrow(),
            phantom: PhantomData,
        }
    }
}

pub struct RefBorrow<'a, T> {
    cell_ref: std::cell::Ref<'a, dyn std::any::Any>,
    phantom: PhantomData<T>,
}

impl<'a, T: 'static> Deref for RefBorrow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.cell_ref.downcast_ref::<T>().unwrap()
    }
}

/// For direct use with [crate::view::Format].
impl<T: Display + 'static> Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.signal_id.register_reactive_dependency();

        REGISTRY.with_borrow(|registry| {
            let ref_cell = registry.state_values.get(&self.signal_id).unwrap();
            let borrow = ref_cell.borrow();
            borrow.downcast_ref::<T>().unwrap().fmt(f)
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
            registry
                .state_values
                .insert(self.signal_id, Rc::new(RefCell::new(value)));
        });

        self.send();
    }

    pub fn update(&self, func: impl Fn(&mut T)) {
        REGISTRY.with_borrow(|registry| {
            let ref_cell = registry.state_values.get(&self.signal_id).unwrap();
            let mut borrow = ref_cell.borrow_mut();

            func(borrow.downcast_mut::<T>().unwrap());
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
