use crate::{platform::Cursor, Children, Platform, View};

impl<P: Platform> View<P> for () {
    type State = ();

    fn init(self, cursor: &mut P::Cursor) {
        cursor.empty();
    }
    fn diff(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl<P: Platform> Children<P> for () {
    type State = ();

    fn init(self, _cursor: &mut P::Cursor) {}
    fn diff(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}
