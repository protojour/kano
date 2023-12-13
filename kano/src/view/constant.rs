use crate::{markup::Markup, View};

/// A constant view, that is never supposed to change internally once rendered.
pub struct Const<V>(pub V);

impl<P, M: Markup<P>, V: View<P, M>> View<P, M> for Const<V> {
    type ConstState = V::ConstState;
    type DiffState = V::ConstState;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        // TODO: Add cursor API to tell the platform that a constant subtree is being entered,
        // so the platform can solve this in the most performant way.
        V::init_const(self.0, cursor)
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        V::init_const(self.0, cursor)
    }

    fn diff(self, _state: &mut Self::DiffState, _cursor: &mut M::Cursor) {
        // The whole point of Const is to skip the diff!
    }
}
