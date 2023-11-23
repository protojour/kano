use std::{
    any::Any,
    sync::{Arc, Mutex, Weak},
    time::Duration,
};

use crate::{
    platform::Platform,
    pubsub::{new_subscriber_id, with_active_notifier, Notify},
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

        let notify = NotificationReceiver {
            weak: Arc::downgrade(&reactive_state.subscriber_state.shared_state),
            subscriber_id: reactive_state.subscriber_state.subscriber_id,
        };

        // Test: Update it at an interval as long as it's alive
        P::spawn_task(async move {
            loop {
                gloo_timers::future::sleep(Duration::from_secs(1)).await;
                if !notify.notify() {
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
        let subscriber_state = SubscriberState {
            shared_state: Arc::new(Mutex::new(None)),
            subscriber_id: new_subscriber_id(),
        };

        {
            let notification_receiver = NotificationReceiver {
                weak: Arc::downgrade(&subscriber_state.shared_state),
                subscriber_id: subscriber_state.subscriber_id,
            };

            let state = with_active_notifier(Box::new(notification_receiver), || {
                update_view(None, cursor)
            });

            let mut lock = subscriber_state.shared_state.lock().unwrap();
            *lock = Some(SharedState {
                current_state: Some(state),
                update_view: Box::new(update_view),
                cursor: box_cursor(cursor),
            });
        }

        Self {
            subscriber_state: subscriber_state,
        }
    }
}

struct SubscriberState<T: Diff> {
    shared_state: Arc<Mutex<Option<SharedState<T>>>>,
    subscriber_id: u64,
}

impl<T: Diff> Clone for SubscriberState<T> {
    fn clone(&self) -> Self {
        Self {
            shared_state: self.shared_state.clone(),
            subscriber_id: self.subscriber_id,
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

struct NotificationReceiver<T: Diff> {
    weak: Weak<Mutex<Option<SharedState<T>>>>,
    subscriber_id: u64,
}

impl<T: Diff + 'static> Notify for NotificationReceiver<T> {
    fn subscriber_id(&self) -> u64 {
        self.subscriber_id
    }

    fn notify(&self) -> bool {
        if let Some(arc) = self.weak.upgrade() {
            let next_receiver = NotificationReceiver {
                weak: self.weak.clone(),
                subscriber_id: self.subscriber_id,
            };

            with_active_notifier(Box::new(next_receiver), || {
                let mut lock = arc.lock().unwrap();
                if let Some(shared_state) = &mut *lock {
                    shared_state.update_view();
                }
                true
            })
        } else {
            false
        }
    }
}
