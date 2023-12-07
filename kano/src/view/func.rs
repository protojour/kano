use crate::{
    platform::Platform,
    registry::{Registry, REGISTRY},
    view_id::ViewId,
    View,
};

pub struct Func<F, A>(pub F, pub A);

impl<P: Platform, T: View<P>, F: (FnOnce() -> T) + 'static> View<P> for Func<F, ()> {
    type State = FuncState<P, T>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let Func(func, ()) = self;
        let view_id = REGISTRY.with_borrow_mut(Registry::alloc_view_id);
        let state = view_id.as_current_func(|| func().init(cursor));
        FuncState { state, view_id }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        let Func(func, ()) = self;
        state.view_id.as_current_func(|| {
            func().diff(&mut state.state, cursor);
        });
    }
}

macro_rules! tuples {
    ($(($a:ident, $i:tt)),+) => {
        impl<P: Platform, T: View<P>, $($a),+, F: (FnOnce($($a),+) -> T) + 'static>  View<P> for Func<F, ($($a),+,)> {
            type State = FuncState<P, T>;

            fn init(self, cursor: &mut P::Cursor) -> Self::State {
                let Func(func, args) = self;
                let view_id = REGISTRY.with_borrow_mut(Registry::alloc_view_id);
                let state = view_id.as_current_func(|| func($(args.$i),+,).init(cursor));
                FuncState { state, view_id }
            }

            fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
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

pub struct FuncState<P: Platform, T: View<P>> {
    view_id: ViewId,
    state: T::State,
}

impl<P: Platform, T: View<P>> Drop for FuncState<P, T> {
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

        let func_state = <Func<_, _> as View<TestPlatform>>::init(
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
