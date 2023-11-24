use std::{
    any::Any,
    sync::{Arc, Mutex, Weak},
};

use crate::{
    platform::Platform,
    pubsub::{OnSignal, SignalId, Subscriber},
    registry::ReactiveId,
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
    shared_state: Arc<Mutex<Option<SharedState<T>>>>,
    subscriber_keepalive: Subscriber,
}

impl<T: Diff> Clone for ReactiveState<T> {
    fn clone(&self) -> Self {
        Self {
            shared_state: self.shared_state.clone(),
            subscriber_keepalive: self.subscriber_keepalive.clone(),
        }
    }
}

impl<T: Diff + 'static> ReactiveState<T> {
    fn new(
        update_view: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        cursor: &mut dyn Any,
        box_cursor: &dyn Fn(&mut dyn Any) -> Box<dyn Any>,
    ) -> Self {
        let reactive_id = ReactiveId::alloc();
        let shared_state: Arc<Mutex<Option<SharedState<T>>>> = Arc::new(Mutex::new(None));
        let subscriber = Subscriber::new(
            reactive_id,
            Arc::new(SignalHandler {
                weak_handle: Arc::downgrade(&shared_state),
            }),
        );

        {
            let state = subscriber
                .id()
                .invoke_as_current(|| update_view(None, cursor));

            // let mut lock = subscriber_state.shared_state.lock().unwrap();
            let mut lock = shared_state.lock().unwrap();
            *lock = Some(SharedState {
                current_state: Some(state),
                update_view: Box::new(update_view),
                cursor: box_cursor(cursor),
            });
        }

        Self {
            shared_state,
            subscriber_keepalive: subscriber,
        }
    }
}

struct SharedState<T: Diff> {
    current_state: Option<T::State>,
    update_view: Box<dyn Fn(Option<T::State>, RefMutDynCursor) -> T::State>,
    cursor: Box<dyn Any>,
}

impl<T: Diff> SharedState<T> {
    fn update_view(&mut self) {
        let old_state = self.current_state.take();
        let new_state = (self.update_view)(old_state, self.cursor.as_mut());
        self.current_state = Some(new_state);
    }
}

struct SignalHandler<T: Diff> {
    weak_handle: Weak<Mutex<Option<SharedState<T>>>>,
}

impl<T: Diff + 'static> OnSignal for SignalHandler<T> {
    fn on_signal(&self, _signal_id: SignalId, reactive_id: ReactiveId) -> bool {
        if let Some(strong_handle) = self.weak_handle.upgrade() {
            reactive_id.invoke_as_current(|| {
                let mut shared_state_lock = strong_handle.lock().unwrap();
                if let Some(shared_state) = &mut *shared_state_lock {
                    shared_state.update_view();
                }
                true
            })
        } else {
            false
        }
    }
}
