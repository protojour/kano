use crate::{
    markup::Markup,
    registry::{Registry, REGISTRY},
    view_id::ViewId,
    View,
};

pub struct Func<F, A>(pub F, pub A);

impl<P, M: Markup<P>, V: View<P, M>, F: (FnOnce() -> V) + 'static> View<P, M> for Func<F, ()> {
    type ConstState = FuncState<P, M, V>;
    type DiffState = FuncState<P, M, V>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        self.init_diff(cursor)
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        let Func(func, ()) = self;
        let view_id = REGISTRY.with_borrow_mut(Registry::alloc_view_id);
        let state = view_id.as_current_func(|| func().init_diff(cursor));
        FuncState { state, view_id }
    }

    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        let Func(func, ()) = self;
        state.view_id.as_current_func(|| {
            func().diff(&mut state.state, cursor);
        });
    }
}

macro_rules! tuples {
    ($(($a:ident, $i:tt)),+) => {
        impl<P, M: Markup<P>, V: View<P, M>, $($a),+, F: (FnOnce($($a),+) -> V) + 'static>  View<P, M> for Func<F, ($($a),+,)> {
            type ConstState = FuncState<P, M, V>;
            type DiffState = FuncState<P, M, V>;

            fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
                self.init_diff(cursor)
            }

            fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
                let Func(func, args) = self;
                let view_id = REGISTRY.with_borrow_mut(Registry::alloc_view_id);
                let state = view_id.as_current_func(|| func($(args.$i),+,).init_diff(cursor));
                FuncState { state, view_id }
            }

            fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
                let Func(func, args) = self;
                state.view_id.as_current_func(|| {
                    func($(args.$i),+,).diff(&mut state.state, cursor);
                });
            }
        }
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

pub struct FuncState<P, M: Markup<P>, V: View<P, M>> {
    view_id: ViewId,
    state: V::DiffState,
}

impl<P, M: Markup<P>, V: View<P, M>> Drop for FuncState<P, M, V> {
    fn drop(&mut self) {
        REGISTRY.with_borrow_mut(|registry| {
            registry.on_view_dropped(self.view_id);
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        platform::test_platform::TestPlatform,
        prelude::platform::use_state,
        registry::{Registry, REGISTRY},
        View,
    };

    use super::Func;

    #[test]
    fn state_gc() {
        REGISTRY.with_borrow_mut(Registry::reset);

        let func_state = <Func<_, _> as View<TestPlatform, ()>>::init_diff(
            Func(
                || {
                    use_state(|| 42);
                    use_state(|| 42);
                },
                (),
            ),
            &mut (),
        );

        REGISTRY.with_borrow(|registry| {
            let (_view_id, signals) = registry.owned_signals_ordered.iter().next().unwrap();

            assert_eq!(registry.owned_signals_ordered.len(), 1);
            assert_eq!(signals.len(), 2);
            assert_eq!(registry.state_values.len(), 2);
        });

        drop(func_state);

        REGISTRY.with_borrow(|registry| {
            assert!(registry.owned_signals_ordered.is_empty());
            assert!(registry.state_values.is_empty());
        });
    }
}
