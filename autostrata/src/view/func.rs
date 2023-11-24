use crate::{
    platform::Platform,
    registry::{ViewId, REGISTRY},
    Diff, View,
};

pub struct Func<F>(pub F);

impl<P: Platform, T: Diff<P>, F: (FnOnce() -> T) + 'static> Diff<P> for Func<F> {
    type State = FuncState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let view_id = ViewId::alloc();
        let state = view_id.invoke_as_current_func_view(|| self.0().init(cursor));
        FuncState { state, view_id }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        state.view_id.invoke_as_current_func_view(|| {
            self.0().diff(&mut state.state, cursor);
        });
    }
}

impl<P: Platform, T: View<P>, F: (FnOnce() -> T) + 'static> View<P> for Func<F> {}

pub struct FuncState<P: Platform, T: Diff<P>> {
    view_id: ViewId,
    state: T::State,
}

impl<P: Platform, T: Diff<P>> Drop for FuncState<P, T> {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_view_dropped(self.view_id);
        });
    }
}
