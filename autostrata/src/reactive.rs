use std::{
    any::Any,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use crate::{
    platform::Platform,
    pubsub::{
        with_current_reactive_subscriber, Notify, Signal, SignalId, Subscriber, SubscriberId,
    },
    Attr, Diff, ViewState,
};

pub struct Reactive<F>(pub F);

impl<T: Diff + 'static, F: (Fn() -> T) + 'static> Diff for Reactive<F> {
    type State = ReactiveState<T>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let reactive_state = ReactiveState::new(
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
        );

        let notify = ReactiveCallback {
            weak_handle: Arc::downgrade(&reactive_state.subscriber_state.shared_state),
        };
        let subscriber_id = reactive_state.subscriber_state.subscriber.subscriber_id();

        // Test: Update it at an interval as long as it's alive
        P::spawn_task(async move {
            let signal = Signal::new();
            loop {
                gloo_timers::future::sleep(Duration::from_secs(1)).await;
                if !notify.notify(signal.signal_id(), subscriber_id) {
                    return;
                }
            }
        });

        reactive_state
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let _old_state = std::mem::replace(state, self.init::<P>(cursor));
    }
}

impl<T: Attr + 'static, F: (Fn() -> T) + 'static> Attr for Reactive<F> {}

impl<T: Diff> ViewState for ReactiveState<T> where T::State: ViewState {}

type RefMutDynCursor<'a> = &'a mut dyn Any;

pub struct ReactiveState<T: Diff> {
    subscriber_state: SubscriberState<T>,
}

impl<T: Diff> Clone for ReactiveState<T> {
    fn clone(&self) -> Self {
        Self {
            subscriber_state: self.subscriber_state.clone(),
        }
    }
}

impl<T: Diff + 'static> ReactiveState<T> {
    fn new(
        update_view: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        cursor: &mut dyn Any,
        box_cursor: &dyn Fn(&mut dyn Any) -> Box<dyn Any>,
    ) -> Self {
        let shared_state: Arc<Mutex<Option<SharedState<T>>>> = Arc::new(Mutex::new(None));
        let subscriber = Subscriber::new(Arc::new(ReactiveCallback {
            weak_handle: Arc::downgrade(&shared_state),
        }));

        {
            let state = with_current_reactive_subscriber(subscriber.subscriber_id(), || {
                update_view(None, cursor)
            });

            // let mut lock = subscriber_state.shared_state.lock().unwrap();
            let mut lock = shared_state.lock().unwrap();
            *lock = Some(SharedState {
                current_state: Some(state),
                update_view: Box::new(update_view),
                cursor: box_cursor(cursor),
            });
        }

        Self {
            subscriber_state: SubscriberState {
                shared_state,
                subscriber,
            },
        }
    }
}

struct SubscriberState<T: Diff> {
    shared_state: Arc<Mutex<Option<SharedState<T>>>>,
    subscriber: Subscriber,
}

impl<T: Diff> Clone for SubscriberState<T> {
    fn clone(&self) -> Self {
        Self {
            shared_state: self.shared_state.clone(),
            subscriber: self.subscriber.clone(),
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

struct ReactiveCallback<T: Diff> {
    weak_handle: Weak<Mutex<Option<SharedState<T>>>>,
}

impl<T: Diff + 'static> Notify for ReactiveCallback<T> {
    fn notify(&self, _signal_id: SignalId, subscriber_id: SubscriberId) -> bool {
        if let Some(strong_handle) = self.weak_handle.upgrade() {
            with_current_reactive_subscriber(subscriber_id, || {
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
