use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    platform::{Cursor, Platform},
    registry::{ViewId, REGISTRY},
    signal::{OnSignal, Signal},
    Attr, Diff, View,
};

/// Reactive wraps a function `F` that produces something diffable,
/// and automatically connects the signals used within the function to automatic updates.
pub struct Reactive<F>(pub F);

impl<P: Platform, T: Diff<P> + 'static, F: (Fn() -> T) + 'static> Diff<P> for Reactive<F> {
    type State = ReactiveState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        ReactiveState::new(
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
            .invoke_as_current_reactive_view(|| (self.0)().init(cursor));
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

impl<P: Platform, T: Diff<P> + 'static> ReactiveState<P, T> {
    fn new(
        update_func: impl Fn(Option<T::State>, &mut P::Cursor) -> T::State + 'static,
        cursor: &mut P::Cursor,
    ) -> Self {
        let view_id = ViewId::alloc();

        // Initialize this to None..
        let data_cell: Rc<RefCell<Option<Data<P, T>>>> = Rc::new(RefCell::new(None));

        // ..so we can make a weak reference to the cell
        // for the reactive callback (it should not own the view).
        REGISTRY.with_borrow_mut(|registry| {
            registry.reactive_callbacks.insert(
                view_id,
                Rc::new(UpdateCallback {
                    weak_data_cell: Rc::downgrade(&data_cell),
                }),
            );
        });

        // Perform the initial "hydration" while registering reactive subscriptions
        let actual_state = view_id.invoke_as_current_reactive_view(|| update_func(None, cursor));

        // Now all information is ready to store the data, including the cursor.
        *data_cell.borrow_mut() = Some(Data {
            actual_state: Some(actual_state),
            update_func: Box::new(update_func),
            cursor: cursor.clone(),
        });

        Self { view_id, data_cell }
    }
}

impl<P: Platform, T: Diff<P>> Drop for ReactiveState<P, T> {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_reactive_dropped(self.view_id);
            registry.on_view_dropped(self.view_id);
        });
    }
}

/// All information needed to reactively update the view is stored here
struct Data<P: Platform, T: Diff<P>> {
    actual_state: Option<T::State>,
    update_func: Box<dyn Fn(Option<T::State>, &mut P::Cursor) -> T::State>,
    cursor: P::Cursor,
}

impl<P: Platform, T: Diff<P>> Data<P, T> {
    fn update(&mut self, view_id: ViewId) {
        let old_state = self.actual_state.take();

        let new_state = view_id
            .invoke_as_current_reactive_view(|| (self.update_func)(old_state, &mut self.cursor));

        self.actual_state = Some(new_state);
    }
}

struct UpdateCallback<P: Platform, T: Diff<P>> {
    weak_data_cell: Weak<RefCell<Option<Data<P, T>>>>,
}

impl<P: Platform, T: Diff<P> + 'static> OnSignal for UpdateCallback<P, T> {
    fn on_signal(&self, _signal: Signal, view_id: ViewId) -> bool {
        if let Some(strong_data_cell) = self.weak_data_cell.upgrade() {
            let mut data_borrow = strong_data_cell.borrow_mut();
            data_borrow.as_mut().unwrap().update(view_id);

            true
        } else {
            false
        }
    }
}
