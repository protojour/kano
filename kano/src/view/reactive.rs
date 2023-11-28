use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    platform::{Cursor, Platform},
    registry::{ViewCallback, REGISTRY},
    view_id::ViewId,
    Attr, Diff, View,
};

/// Reactive wraps a function `F` that produces something diffable,
/// and automatically connects the signals used within the function to automatic updates.
pub struct Reactive<F>(pub F);

impl<P: Platform, T: Diff<P> + 'static, F: (Fn() -> T) + 'static> Diff<P> for Reactive<F> {
    type State = ReactiveState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        mk_reactive_state(
            move |prev_state, cursor| {
                if let Some(mut state) = prev_state {
                    (self.0)().diff(&mut state, cursor);
                    state
                } else {
                    (self.0)().init(cursor)
                }
            },
            cursor,
        )
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        cursor.enter_diff();
        let new_state = state
            .view_id
            .as_current_reactive(|| (self.0)().init(cursor));
        cursor.exit_diff();

        let mut data_cell = state.data_cell.borrow_mut();
        if let Some(data) = data_cell.as_mut() {
            data.cursor = cursor.clone();
            data.actual_state = Some(new_state);
        }
    }
}

impl<P: Platform, T: Diff<P> + 'static, F: (Fn() -> T) + 'static> View<P> for Reactive<F> {}
impl<P: Platform, T: Attr<P> + 'static, F: (Fn() -> T) + 'static> Attr<P> for Reactive<F> {}

pub struct ReactiveState<P: Platform, T: Diff<P>> {
    view_id: ViewId,
    data_cell: Rc<RefCell<Option<Data<P, T>>>>,
}

struct Data<P: Platform, T: Diff<P>> {
    actual_state: Option<T::State>,
    update_func: Box<dyn Fn(Option<T::State>, &mut P::Cursor) -> T::State>,
    cursor: P::Cursor,
}

impl<P: Platform, T: Diff<P>> Drop for ReactiveState<P, T> {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_reactive_dropped(self.view_id);
            registry.on_view_dropped(self.view_id);
        });
    }
}

fn mk_reactive_state<P: Platform, T: Diff<P> + 'static>(
    update_func: impl Fn(Option<T::State>, &mut P::Cursor) -> T::State + 'static,
    cursor: &mut P::Cursor,
) -> ReactiveState<P, T> {
    let (view_id, data_cell) = REGISTRY.with_borrow_mut(|registry| {
        let view_id = registry.alloc_view_id();

        // Initialize this to None..
        let data_cell: Rc<RefCell<Option<Data<P, T>>>> = Rc::new(RefCell::new(None));

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
        update_func: Box::new(update_func),
        cursor: cursor.clone(),
    });

    ReactiveState { view_id, data_cell }
}

fn mk_reactive_callback<P: Platform, T: Diff<P> + 'static>(
    weak_data_cell: Weak<RefCell<Option<Data<P, T>>>>,
) -> ViewCallback {
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
        Diff,
    };

    use super::Reactive;

    #[test]
    fn subscription_gc() {
        REGISTRY.with_borrow_mut(Registry::reset);

        let test_sig = Signal(1337);

        let state0 = <Reactive<_> as Diff<TestPlatform>>::init(
            Reactive(move || {
                use_state(|| ()); // owned state
                test_sig.register_reactive_dependency();
            }),
            &mut (),
        );
        let _state1 = <Reactive<_> as Diff<TestPlatform>>::init(
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

        let call_count = Rc::new(RefCell::new(0));

        let _state = {
            let call_count = call_count.clone();
            <Reactive<_> as Diff<TestPlatform>>::init(
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
                registry.peek_next_signal_id(),
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
                registry.peek_next_signal_id(),
                1,
                "No signal has been allocated after callback invoke"
            );
        });
    }
}
