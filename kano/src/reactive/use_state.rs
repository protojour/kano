use std::{cell::RefCell, fmt::Display, marker::PhantomData, ops::Deref, rc::Rc};

use crate::{registry::REGISTRY, signal::Signal};

pub fn use_state<T: 'static>(init_func: impl FnOnce() -> T) -> State<T> {
    let signal = REGISTRY.with_borrow_mut(|registry| {
        let (signal, reused) = registry.alloc_or_reuse_func_view_signal();

        // If the signal is reused, the value should already be in the registry,
        // and we should not reset the state.
        if !reused {
            registry
                .state_values
                .insert(signal, Rc::new(RefCell::new(init_func())));
        }

        signal
    });

    State {
        signal,
        phantom: PhantomData,
    }
}

pub struct State<T> {
    signal: Signal,
    phantom: PhantomData<T>,
}

impl<T: 'static> State<T> {
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.signal.register_reactive_dependency();

        let ref_cell = REGISTRY
            .with_borrow(|registry| registry.state_values.get(&self.signal).unwrap().clone());
        let borrow = ref_cell.borrow();
        let value_ref = borrow.downcast_ref::<T>().unwrap();

        value_ref.clone()
    }

    pub fn get_ref(&self) -> Ref<T> {
        self.signal.register_reactive_dependency();

        let ref_cell = REGISTRY
            .with_borrow(|registry| registry.state_values.get(&self.signal).unwrap().clone());
        Ref {
            ref_cell,
            phantom: PhantomData,
        }
    }

    pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.signal.register_reactive_dependency();

        REGISTRY.with_borrow(|registry| {
            f(registry
                .state_values
                .get(&self.signal)
                .unwrap()
                .borrow()
                .downcast_ref::<T>()
                .unwrap())
        })
    }

    pub fn set(&self, value: T) {
        REGISTRY.with_borrow_mut(|registry| {
            registry
                .state_values
                .insert(self.signal, Rc::new(RefCell::new(value)));
        });

        self.signal.send();
    }

    pub fn update(&self, func: impl Fn(&mut T)) {
        REGISTRY.with_borrow(|registry| {
            let ref_cell = registry.state_values.get(&self.signal).unwrap();
            let mut borrow = ref_cell.borrow_mut();

            func(borrow.downcast_mut::<T>().unwrap());
        });

        self.signal.send();
    }
}

impl State<bool> {
    pub fn toggle(&self) {
        self.update(|value| *value = !*value);
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal,
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for State<T> {}

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

/// For direct use with [crate::view::Fmt].
impl<T: Display + 'static> Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.signal.register_reactive_dependency();

        REGISTRY.with_borrow(|registry| {
            let ref_cell = registry.state_values.get(&self.signal).unwrap();
            let borrow = ref_cell.borrow();
            borrow.downcast_ref::<T>().unwrap().fmt(f)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::use_state;

    #[test]
    #[should_panic = "state should not be used outside the view hierarchy!"]
    fn use_state_outside_view() {
        use_state(|| 666);
    }
}
