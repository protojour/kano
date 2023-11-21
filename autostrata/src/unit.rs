use crate::{AttrSet, Children, Diff, Platform, Unmount, ViewState};

impl Diff for () {
    type State = ();

    fn init<P: Platform>(self, _cursor: &mut P::Cursor) -> () {}
    fn diff<P: Platform>(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl Unmount for () {
    fn unmount<P: Platform>(&mut self, _cursor: &mut P::Cursor) {}
}

impl Children for () {}
impl AttrSet for () {}
impl ViewState for () {}
