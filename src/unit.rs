use crate::{AttrSet, Diff, List, Platform, Unmount};

impl Diff for () {
    type State = ();

    fn init<P: Platform>(self, _cursor: &mut P::Cursor) -> () {
        ()
    }

    fn diff<P: Platform>(self, _: &mut Self::State, _cursor: &mut P::Cursor) {}
}

impl List for () {}
impl AttrSet for () {}
impl Unmount for () {
    fn unmount<P: Platform>(&mut self, _cursor: &mut P::Cursor) {}
}
