#![allow(clippy::type_complexity)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    markup::Markup,
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
    type ConstState = ReactiveState<P, M, V>;
    type DiffState = ReactiveState<P, M, V>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        self.init_diff(cursor)
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        let func = self.0;
        mk_reactive_state(
            Box::new(move |prev_state, cursor| {
                if let Some(mut state) = prev_state {
                    func().diff(&mut state, cursor);
                    state
                } else {
                    func().init_diff(cursor)
                }
            }),
            cursor,
        )
    }

    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        let view_id = state.view_id;
        let mut data_cell = state.data_cell.borrow_mut();
        if let Some(data) = data_cell.as_mut() {
            if let Some(actual_state) = data.actual_state.as_mut() {
                view_id.as_current_reactive(|| (self.0)().diff(actual_state, cursor));
            } else {
                panic!("No actual state");
            }
        } else {
            panic!("No data cell");
        }
    }
}

pub struct ReactiveState<P, M: Markup<P>, V: View<P, M>> {
    view_id: ViewId,
    data_cell: Rc<RefCell<Option<Data<P, M, V>>>>,
}
impl<P, M, V> ReactiveState<P, M, V>
where
    P: 'static,
    M: Markup<P>,
    V: View<P, M> + 'static,
{
    pub fn update_fn(&self) -> impl (Fn() -> bool) + Clone {
        let view_id = self.view_id;
        let callback = mk_reactive_callback(Rc::downgrade(&self.data_cell));
        move || callback(view_id)
    }
}

impl<P, M, V> Clone for ReactiveState<P, M, V>
where
    M: Markup<P>,
    V: View<P, M>,
{
    fn clone(&self) -> Self {
        Self {
            view_id: self.view_id,
            data_cell: self.data_cell.clone(),
        }
    }
}

struct Data<P, M, V>
where
    M: Markup<P>,
    V: View<P, M>,
{
    actual_state: Option<V::DiffState>,
    update_func: Box<dyn Fn(Option<V::DiffState>, &mut M::Cursor) -> V::DiffState>,
    cursor: M::Cursor,
}

impl<P, M, V> Drop for ReactiveState<P, M, V>
where
    M: Markup<P>,
    V: View<P, M>,
{
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_reactive_dropped(self.view_id);
            registry.on_view_dropped(self.view_id);
        });
    }
}

fn mk_reactive_state<P, M, V>(
    update_func: Box<dyn Fn(Option<V::DiffState>, &mut M::Cursor) -> V::DiffState>,
    cursor: &mut M::Cursor,
) -> ReactiveState<P, M, V>
where
    P: 'static,
    M: Markup<P>,
    V: View<P, M> + 'static,
{
    let (view_id, data_cell) = REGISTRY.with_borrow_mut(|registry| {
        let view_id = registry.alloc_view_id();

        // Initialize this to None..
        let data_cell: Rc<RefCell<Option<Data<P, M, V>>>> = Rc::new(RefCell::new(None));

        // ..so we can make a weak reference to the cell
        // for the reactive callback (it should not own the view).
        registry.add_reactive_view(view_id, mk_reactive_callback(Rc::downgrade(&data_cell)));

        (view_id, data_cell.clone())
    });

    // Perform the initial "hydration" while registering reactive subscriptions
    let actual_state = view_id.as_current_reactive(|| update_func(None, cursor));

    // Now all information is ready to store the data, including the cursor.
    *data_cell.borrow_mut() = Some(Data {
        actual_state: Some(actual_state),
        update_func,
        cursor: cursor.clone(),
    });

    ReactiveState { view_id, data_cell }
}

fn mk_reactive_callback<P, M, V>(
    weak_data_cell: Weak<RefCell<Option<Data<P, M, V>>>>,
) -> ViewCallback
where
    P: 'static,
    M: Markup<P>,
    V: View<P, M> + 'static,
{
    Rc::new(move |view_id| {
        let Some(strong_data_cell) = weak_data_cell.upgrade() else {
            return false;
        };

        let mut data_mut_borrow = strong_data_cell.borrow_mut();
        let data = Option::unwrap(data_mut_borrow.as_mut());

        {
            let old_state = data.actual_state.take();

            let new_state =
                view_id.as_current_reactive(|| (data.update_func)(old_state, &mut data.cursor));

            data.actual_state = Some(new_state);
        }

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
