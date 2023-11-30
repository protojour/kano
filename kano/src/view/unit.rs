use crate::{platform::Cursor, Children, Diff, Platform, View};

impl<P: Platform> Diff<P> for () {
    type State = ();

    fn init(self, cursor: &mut P::Cursor) {
        cursor.empty();
    }
    fn diff(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl<P: Platform> Children<P> for () {}
impl<P: Platform> View<P> for () {}
