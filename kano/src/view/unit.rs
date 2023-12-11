use crate::{
    markup::{Cursor, Markup},
    Children, View,
};

impl<P, M: Markup<P>> View<P, M> for () {
    type State = ();

    fn init(self, cursor: &mut M::Cursor) {
        cursor.empty();
    }
    fn diff(self, _: &mut Self::State, _cursor: &mut M::Cursor) {}
}

impl<P, M: Markup<P>> Children<P, M> for () {
    type State = ();

    fn init(self, _cursor: &mut M::Cursor) {}
    fn diff(self, _: &mut Self::State, _cursor: &mut M::Cursor) {}
}
