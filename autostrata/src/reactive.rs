use std::{
    any::Any,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{platform::Platform, Attr, Diff, ViewState};

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

        let weak_shared = Arc::downgrade(&reactive_state.shared);

        // Test: Update it at an interval as long as it's alive
        P::spawn_task(async move {
            loop {
                gloo_timers::future::sleep(Duration::from_secs(1)).await;
                if let Some(shared) = weak_shared.upgrade() {
                    let mut lock = shared.lock().unwrap();
                    lock.update();
                } else {
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

type RefMutDynCursor<'a> = &'a mut dyn Any;

pub struct ReactiveState<T: Diff> {
    shared: Arc<Mutex<SharedState<T>>>,
}

impl<T: Diff> Clone for ReactiveState<T> {
    fn clone(&self) -> Self {
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T: Diff> ReactiveState<T> {
    fn new(
        func: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        cursor: &mut dyn Any,
        box_cursor: &dyn Fn(&mut dyn Any) -> Box<dyn Any>,
    ) -> Self {
        let state = func(None, cursor);
        let cursor = box_cursor(cursor);
        let shared = Arc::new(Mutex::new(SharedState {
            state: Some(state),
            func: Box::new(func),
            cursor,
        }));

        Self { shared }
    }
}

struct SharedState<T: Diff> {
    state: Option<T::State>,
    func: Box<dyn Fn(Option<T::State>, RefMutDynCursor) -> T::State>,
    cursor: Box<dyn Any>,
}

impl<T: Diff> SharedState<T> {
    fn update(&mut self) {
        let state = self.state.take();
        let new_state = (self.func)(state, self.cursor.as_mut());
        self.state = Some(new_state);
    }
}

impl<T: Diff> ViewState for ReactiveState<T> where T::State: ViewState {}
