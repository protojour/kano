use crate::{
    platform::Platform,
    registry::{ViewId, REGISTRY},
    Diff, View,
};

pub struct Func<F, A>(pub F, pub A);

impl<P: Platform, T: Diff<P>, F: (FnOnce() -> T) + 'static> Diff<P> for Func<F, ()> {
    type State = FuncState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let Func(func, ()) = self;
        let view_id = ViewId::alloc();
        let state = view_id.invoke_as_current_func_view(|| func().init(cursor));
        FuncState { state, view_id }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let Func(func, ()) = self;
        state.view_id.invoke_as_current_func_view(|| {
            func().diff(&mut state.state, cursor);
        });
    }
}

impl<P: Platform, T: View<P>, F: (FnOnce() -> T) + 'static> View<P> for Func<F, ()> {}

macro_rules! tuples {
    ($(($a:ident, $i:tt)),+) => {
        impl<P: Platform, T: Diff<P>, $($a),+, F: (FnOnce($($a),+) -> T) + 'static>  Diff<P> for Func<F, ($($a),+,)> {
            type State = FuncState<P, T>;

            fn init(self, cursor: &mut P::Cursor) -> Self::State {
                let Func(func, args) = self;
                let view_id = ViewId::alloc();
                let state = view_id.invoke_as_current_func_view(|| func($(args.$i),+,).init(cursor));
                FuncState { state, view_id }
            }

            fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
                let Func(func, args) = self;
                state.view_id.invoke_as_current_func_view(|| {
                    func($(args.$i),+,).diff(&mut state.state, cursor);
                });
            }
        }

        impl<P: Platform, T: View<P>, $($a),+, F: (FnOnce($($a),+) -> T) + 'static> View<P> for Func<F, ($($a),+,)> {}
    }
}

tuples!((A0, 0));
tuples!((A0, 0), (A1, 1));
tuples!((A0, 0), (A1, 1), (A2, 2));
tuples!((A0, 0), (A1, 1), (A2, 2), (A3, 3));
tuples!((A0, 0), (A1, 1), (A2, 2), (A3, 3), (A4, 4));
tuples!((A0, 0), (A1, 1), (A2, 2), (A3, 3), (A4, 4), (A5, 5));
tuples!(
    (A0, 0),
    (A1, 1),
    (A2, 2),
    (A3, 3),
    (A4, 4),
    (A5, 5),
    (A6, 6)
);

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