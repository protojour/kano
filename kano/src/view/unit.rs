use crate::{
    markup::{Cursor, Markup},
    Children, View,
};

impl<P, M: Markup<P>> View<P, M> for () {
    type ConstState = ();
    type DiffState = ();

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        cursor.empty();
    }

    fn init_diff(self, cursor: &mut M::Cursor) {
        cursor.empty();
    }

    fn diff(self, _: &mut Self::DiffState, _cursor: &mut M::Cursor) {}
}

impl<P, M: Markup<P>> Children<P, M> for () {
    type ConstState = ();
    type DiffState = ();

    fn init_const(self, _cursor: &mut M::Cursor) -> Self::ConstState {}
    fn init_diff(self, _cursor: &mut M::Cursor) {}
    fn diff(self, _: &mut Self::DiffState, _cursor: &mut M::Cursor) {}
}
