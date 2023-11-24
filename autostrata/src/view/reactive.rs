use std::{
    any::Any,
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    platform::Platform,
    pubsub::{OnSignal, SignalId},
    registry::{ViewId, REGISTRY},
    Attr, Diff, ViewState,
};

pub struct Reactive<F>(pub F);

impl<T: Diff + 'static, F: (Fn() -> T) + 'static> Diff for Reactive<F> {
    type State = ReactiveState<T>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        ReactiveState::new(
            move |prev_state, cursor| {
                let cursor = cursor.downcast_mut::<P::Cursor>().unwrap();

                P::debug_start_reactive_update(cursor);

                let value = (self.0)();
                if let Some(mut state) = prev_state {
                    value.diff::<P>(&mut state, cursor);
                    state
                } else {
                    value.init::<P>(cursor)
                }
            },
            cursor,
            &|cursor| Box::new(cursor.downcast_mut::<P::Cursor>().unwrap().clone()),
        )
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let _old_state = std::mem::replace(state, self.init::<P>(cursor));
    }
}

impl<T: Attr + 'static, F: (Fn() -> T) + 'static> Attr for Reactive<F> {}

impl<T: Diff> ViewState for ReactiveState<T> where T::State: ViewState {}

type RefMutDynCursor<'a> = &'a mut dyn Any;

pub struct ReactiveState<T: Diff> {
    view_id: ViewId,
    _data_cell: Rc<RefCell<Option<Data<T>>>>,
}

impl<T: Diff + 'static> ReactiveState<T> {
    fn new(
        update_view: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        cursor: &mut dyn Any,
        box_cursor: &dyn Fn(&mut dyn Any) -> Box<dyn Any>,
    ) -> Self {
        let view_id = ViewId::alloc();

        // Initialize this to None..
        let data_cell: Rc<RefCell<Option<Data<T>>>> = Rc::new(RefCell::new(None));

        // ..so we can make a weak reference to the cell
        // for the reactive callback (it should not own the view).
        REGISTRY.with_borrow_mut(|registry| {
            registry.reactive_callbacks.insert(
                view_id,
                Rc::new(SignalHandler {
                    weak_data_cell: Rc::downgrade(&data_cell),
                }),
            );
        });

        {
            let actual_state = view_id.invoke_as_current_reactive(|| update_view(None, cursor));

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

impl<T: Diff> Drop for ReactiveState<T> {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.remove_subscriber(self.view_id);
        });
    }
}

/// All information needed to reactively update the view is stored here
struct Data<T: Diff> {
    actual_state: Option<T::State>,
    update_view: Box<dyn Fn(Option<T::State>, RefMutDynCursor) -> T::State>,
    cursor: Box<dyn Any>,
}

impl<T: Diff> Data<T> {
    fn update_view(&mut self) {
        let old_state = self.actual_state.take();
        let new_state = (self.update_view)(old_state, self.cursor.as_mut());
        self.actual_state = Some(new_state);
    }
}

struct SignalHandler<T: Diff> {
    weak_data_cell: Weak<RefCell<Option<Data<T>>>>,
}

impl<T: Diff + 'static> OnSignal for SignalHandler<T> {
    fn on_signal(&self, _signal_id: SignalId, view_id: ViewId) -> bool {
        if let Some(strong_handle) = self.weak_data_cell.upgrade() {
            view_id.invoke_as_current_reactive(|| {
                if let Some(data) = &mut *strong_handle.borrow_mut() {
                    data.update_view();
                }
                true
            })
        } else {
            false
        }
    }
}
