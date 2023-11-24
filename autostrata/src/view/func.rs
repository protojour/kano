use crate::{Diff, View};

pub struct Func<F>(pub F);

impl<T: Diff, F: (Fn() -> T) + 'static> Diff for Func<F> {
    type State = FuncState<T>;

    fn init<P: crate::platform::Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let state = self.0().init::<P>(cursor);
        FuncState { state }
    }

    fn diff<P: crate::platform::Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        self.0().diff::<P>(&mut state.state, cursor);
    }
}

impl<T: View, F: (Fn() -> T) + 'static> View for Func<F> {}

pub struct FuncState<T: Diff> {
    state: T::State,
}
