use std::any::Any;

use crate::{platform::Platform, Attr, Diff, Unmount, ViewState};

pub struct Reactive<F>(pub F);

impl<T: Diff, F: (Fn() -> T) + 'static> Diff for Reactive<F> {
    type State = ReactiveState<T>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        ReactiveState::new(
            move |prev_state, cursor| {
                let cursor = cursor.downcast_mut::<P::Cursor>().unwrap();

                let value = (self.0)();
                if let Some(mut state) = prev_state {
                    value.diff::<P>(&mut state, cursor);
                    state
                } else {
                    value.init::<P>(cursor)
                }
            },
            Box::new(cursor.clone()),
        )
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let _old_state = std::mem::replace(state, self.init::<P>(cursor));
    }
}

impl<T: Attr, F: (Fn() -> T) + 'static> Attr for Reactive<F> {}

type RefMutDynCursor<'a> = &'a mut dyn Any;

pub struct ReactiveState<T: Diff> {
    state: Option<T::State>,
    func: Box<dyn Fn(Option<T::State>, RefMutDynCursor) -> T::State>,
    cursor: Box<dyn Any>,
}

impl<T: Diff> ReactiveState<T> {
    fn new(
        func: impl Fn(Option<T::State>, RefMutDynCursor) -> T::State + 'static,
        mut cursor: Box<dyn Any>,
    ) -> Self {
        let state = func(None, cursor.as_mut());

        Self {
            state: Some(state),
            func: Box::new(func),
            cursor,
        }
    }

    fn _call_func_works(&mut self) {
        let state = self.state.take();
        let new_state = (self.func)(state, self.cursor.as_mut());
        self.state = Some(new_state);
    }
}

impl<T: Diff> Unmount for ReactiveState<T>
where
    T::State: Unmount,
{
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        if let Some(mut state) = self.state.take() {
            state.unmount::<P>(cursor);
        }
    }
}

impl<T: Diff> ViewState for ReactiveState<T> where T::State: ViewState {}
