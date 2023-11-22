use crate::{platform::Cursor, AttrSet, Children, Diff, Platform, ViewState};

impl Diff for () {
    type State = ();

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> () {
        cursor.empty();
    }
    fn diff<P: Platform>(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl Children for () {}
impl AttrSet for () {}
impl ViewState for () {}
