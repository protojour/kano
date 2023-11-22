use crate::{AttrSet, Children, Diff, Platform, ViewState};

impl Diff for () {
    type State = ();

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> () {
        P::mark_empty(cursor);
    }
    fn diff<P: Platform>(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl Children for () {}
impl AttrSet for () {}
impl ViewState for () {}
