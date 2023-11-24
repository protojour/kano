use std::{
    any::Any,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    platform::Platform,
    pubsub::{OnSignal, SignalId},
    registry::{ViewId, REGISTRY},
    Attr, Diff, View,
};

pub struct Reactive<F>(pub F);

impl<P: Platform, T: Diff<P> + 'static, F: (Fn() -> T) + 'static> Diff<P> for Reactive<F> {
    type State = ReactiveState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        ReactiveState::new(
            move |prev_state, cursor| {
                let cursor = cursor.downcast_mut::<P::Cursor>().unwrap();

                P::debug_start_reactive_update(cursor);

                let value = (self.0)();
                if let Some(mut state) = prev_state {
                    value.diff(&mut state, cursor);
                    state
                } else {
                    value.init(cursor)
                }
            },
            cursor,
            &|cursor| Box::new(cursor.downcast_mut::<P::Cursor>().unwrap().clone()),
        )
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let _old_state = std::mem::replace(state, self.init(cursor));
    }
}

impl<P: Platform, T: Diff<P> + 'static, F: (Fn() -> T) + 'static> View<P> for Reactive<F> {}
impl<P: Platform, T: Attr<P> + 'static, F: (Fn() -> T) + 'static> Attr<P> for Reactive<F> {}

type RefMutDynCursor<'a> = &'a mut dyn Any;

pub struct ReactiveState<P: Platform, T: Diff<P>> {
    view_id: ViewId,
    _data_cell: Rc<RefCell<Option<Data<P, T>>>>,
}

impl<P: Platform, T: Diff<P> + 'static> ReactiveState<P, T> {
    fn new(
        update_view: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        cursor: &mut dyn Any,
        box_cursor: &dyn Fn(&mut dyn Any) -> Box<dyn Any>,
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

        {
            let actual_state =
                view_id.invoke_as_current_reactive_view(|| update_view(None, cursor));

            // Now all information is ready to store the data, including the cursor.
            *data_cell.borrow_mut() = Some(Data {
                actual_state: Some(actual_state),
                update_view: Box::new(update_view),
                cursor: box_cursor(cursor),
            });
        }

        Self {
            view_id,
            _data_cell: data_cell,
        }
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
    update_view: Box<dyn Fn(Option<T::State>, RefMutDynCursor) -> T::State>,
    cursor: Box<dyn Any>,
}

impl<P: Platform, T: Diff<P>> Data<P, T> {
    fn update_view(&mut self, view_id: ViewId) {
        let old_state = self.actual_state.take();

        let new_state = view_id.invoke_as_current_reactive_view(|| {
            (self.update_view)(old_state, self.cursor.as_mut())
        });

        self.actual_state = Some(new_state);
    }
}

struct UpdateCallback<P: Platform, T: Diff<P>> {
    weak_data_cell: Weak<RefCell<Option<Data<P, T>>>>,
}

impl<P: Platform, T: Diff<P> + 'static> OnSignal for UpdateCallback<P, T> {
    fn on_signal(&self, _signal_id: SignalId, view_id: ViewId) -> bool {
        if let Some(strong_data_cell) = self.weak_data_cell.upgrade() {
            let mut data_borrow = strong_data_cell.borrow_mut();
            data_borrow.as_mut().unwrap().update_view(view_id);

            true
        } else {
            false
        }
    }
}
