#![allow(clippy::type_complexity)]

use std::{
    any::Any,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    markup::{Cursor, Markup},
    registry::{ViewCallback, REGISTRY},
    view_id::ViewId,
    View,
};

/// Reactive wraps a function `F` that produces something diffable,
/// and automatically connects the signals used within the function to automatic updates.
pub struct Reactive<F>(pub F);

impl<P, M, V, F> View<P, M> for Reactive<F>
where
    P: 'static,
    M: Markup<P>,
    V: View<P, M> + 'static,
    F: (Fn() -> V) + 'static,
{
    type ConstState = ReactiveState<P, M>;
    type DiffState = ReactiveState<P, M>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        self.init_diff(cursor)
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        let func = self.0;
        mk_reactive_state(
            Box::new(move |prev_state, cursor| {
                // note: This function is called when the reactive view needs to update itself

                if let Some(inner_state) = prev_state {
                    func().diff(inner_state.downcast_mut::<V::DiffState>().unwrap(), cursor);
                    None
                } else {
                    let state = func().init_diff(cursor);
                    Some(Box::new(state))
                }
            }),
            cursor,
        )
    }

    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        // note: This is called when there is an _external_ reason for the reactive view to update.

        let mut data_cell = state.data_cell.borrow_mut();
        let data = data_cell.as_mut().expect("No data cell");

        if let Some(inner_state) = data.boxed_state.downcast_mut::<V::DiffState>() {
            let func = self.0;
            state
                .view_id
                .as_current_reactive(|| func().diff(inner_state, cursor));
        } else {
            drop(data_cell);

            // Since the reactive state is not type-dependent on the inner state `V::DiffState`,
            // We may be in a situation where this is logically a different view than before,
            // but the external state holder was not able to distinguish a type change in state.
            // Now just replace/re-init and the reactive state will receive a new ViewId
            // (this will invoke mk_reactive_state).
            cursor.replace(|cursor| {
                let new_state = self.init_diff(cursor);
                // Overwrite the whole state; this drops the old ViewId:
                *state = new_state;
            });
        }
    }
}

pub struct ReactiveState<P, M: Markup<P>> {
    view_id: ViewId,
    data_cell: Rc<RefCell<Option<Data<P, M>>>>,
}

impl<P, M> ReactiveState<P, M>
where
    P: 'static,
    M: Markup<P>,
{
    pub fn update_fn(&self) -> impl (Fn() -> bool) + Clone {
        let view_id = self.view_id;
        let callback = mk_reactive_callback(Rc::downgrade(&self.data_cell));
        move || callback(view_id)
    }
}

impl<P, M> Clone for ReactiveState<P, M>
where
    M: Markup<P>,
{
    fn clone(&self) -> Self {
        Self {
            view_id: self.view_id,
            data_cell: self.data_cell.clone(),
        }
    }
}

impl<P, M> Drop for ReactiveState<P, M>
where
    M: Markup<P>,
{
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_reactive_dropped(self.view_id);
            registry.on_view_dropped(self.view_id);
        });
    }
}

struct Data<P, M>
where
    M: Markup<P>,
{
    boxed_state: Box<dyn Any>,
    update_func: Box<dyn Fn(Option<&mut dyn Any>, &mut M::Cursor) -> Option<Box<dyn Any>>>,
    cursor: M::Cursor,
}

#[inline(never)]
fn mk_reactive_state<P, M>(
    update_func: Box<dyn Fn(Option<&mut dyn Any>, &mut M::Cursor) -> Option<Box<dyn Any>>>,
    cursor: &mut M::Cursor,
) -> ReactiveState<P, M>
where
    P: 'static,
    M: Markup<P>,
{
    // Initialize this to None..
    let data_cell: Rc<RefCell<Option<Data<P, M>>>> = Rc::new(RefCell::new(None));

    let view_id = REGISTRY.with_borrow_mut(|registry| {
        let view_id = registry.alloc_view_id();

        // ..so we can make a weak reference to the cell
        // for the reactive callback (it should not own the view).
        registry.add_reactive_view(view_id, mk_reactive_callback(Rc::downgrade(&data_cell)));

        view_id
    });

    // Perform the initial "hydration" while registering reactive subscriptions
    let boxed_state = view_id
        .as_current_reactive(|| update_func(None, cursor))
        .expect("no initial state");

    // Now all information is ready to store the data, including the cursor.
    *data_cell.borrow_mut() = Some(Data {
        boxed_state,
        update_func,
        cursor: cursor.clone(),
    });

    ReactiveState { view_id, data_cell }
}

fn mk_reactive_callback<P, M>(weak_data_cell: Weak<RefCell<Option<Data<P, M>>>>) -> ViewCallback
where
    P: 'static,
    M: Markup<P>,
{
    Rc::new(move |view_id| {
        let Some(strong_data_cell) = weak_data_cell.upgrade() else {
            return false;
        };

        let mut data_mut_borrow = strong_data_cell.borrow_mut();
        let data = Option::unwrap(data_mut_borrow.as_mut());

        view_id.as_current_reactive(|| {
            (data.update_func)(Some(data.boxed_state.as_mut()), &mut data.cursor)
        });

        true
    })
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        platform::test_platform::TestPlatform,
        prelude::platform::use_state,
        registry::{Registry, REGISTRY},
        signal::Signal,
        View,
    };

    use super::Reactive;

    #[test]
    fn subscription_gc() {
        REGISTRY.with_borrow_mut(Registry::reset);

        let test_sig = Signal(1337);

        let state0 = <Reactive<_> as View<TestPlatform, ()>>::init_diff(
            Reactive(move || {
                use_state(|| ()); // owned state
                test_sig.register_reactive_dependency();
            }),
            &mut (),
        );
        let _state1 = <Reactive<_> as View<TestPlatform, ()>>::init_diff(
            Reactive(move || {
                use_state(|| ()); // owned state
                test_sig.register_reactive_dependency();
            }),
            &mut (),
        );

        REGISTRY.with_borrow(|registry| {
            let subscriptions = registry.subscriptions_by_signal.get(&test_sig).unwrap();
            assert_eq!(subscriptions.len(), 2);
            assert_eq!(registry.subscriptions_by_view.len(), 2);
            assert_eq!(registry.reactive_entries.len(), 2);
            assert_eq!(registry.owned_signals_ordered.len(), 2);
            assert_eq!(registry.state_values.len(), 2);
        });

        drop(state0);

        REGISTRY.with_borrow(|registry| {
            let subscriptions = registry.subscriptions_by_signal.get(&test_sig).unwrap();
            assert_eq!(subscriptions.len(), 1);
            assert_eq!(registry.subscriptions_by_view.len(), 1);
            assert_eq!(registry.reactive_entries.len(), 1);
            assert_eq!(registry.owned_signals_ordered.len(), 1);
            assert_eq!(registry.state_values.len(), 1);
        });
    }

    #[test]
    fn signal_reuse() {
        REGISTRY.with_borrow_mut(Registry::reset);
        let signal_origin = REGISTRY.with_borrow(|reg| reg.peek_next_signal_id());

        let call_count = Rc::new(RefCell::new(0));

        let _state = {
            let call_count = call_count.clone();
            <Reactive<_> as View<TestPlatform, ()>>::init_diff(
                Reactive(move || {
                    // A dependency on its own state
                    use_state(|| ()).get();

                    *call_count.borrow_mut() += 1;
                }),
                &mut (),
            )
        };

        assert_eq!(*call_count.borrow(), 1);

        let (view_id, callback) = REGISTRY.with_borrow(|registry| {
            assert_eq!(
                registry.peek_next_signal_id() - signal_origin,
                1,
                "One signal has been allocated"
            );

            let (view_id, entry) = registry.reactive_entries.iter().next().unwrap();
            (*view_id, entry.callback.clone())
        });

        // Invoke the callback, which should increase the call count
        callback(view_id);

        assert_eq!(*call_count.borrow(), 2);

        REGISTRY.with_borrow(|registry| {
            assert_eq!(
                registry.peek_next_signal_id() - signal_origin,
                1,
                "No signal has been allocated after callback invoke"
            );
        });
    }
}
