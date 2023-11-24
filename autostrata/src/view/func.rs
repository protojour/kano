use crate::{registry::ViewId, Diff, View};

pub struct Func<F>(pub F);

impl<T: Diff, F: (Fn() -> T) + 'static> Diff for Func<F> {
    type State = FuncState<T>;

    fn init<P: crate::platform::Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let view_id = ViewId::alloc();
        let state = view_id.invoke_as_current_func_view(|| self.0().init::<P>(cursor));
        FuncState { state, view_id }
    }

    fn diff<P: crate::platform::Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        state.view_id.invoke_as_current_func_view(|| {
            self.0().diff::<P>(&mut state.state, cursor);
        });
    }
}

impl<T: View, F: (Fn() -> T) + 'static> View for Func<F> {}

pub struct FuncState<T: Diff> {
    view_id: ViewId,
    state: T::State,
}

impl<T: Diff> Drop for FuncState<T> {
    fn drop(&mut self) {
        // Deregister view
    }
}
