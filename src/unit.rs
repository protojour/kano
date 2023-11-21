use crate::{Diff, List, Renderer, Unmount};

impl Diff for () {
    type State = ();

    fn init<R: Renderer>(self, _cursor: &mut R::Cursor) -> () {
        ()
    }

    fn diff<R: Renderer>(self, _: &mut Self::State, _cursor: &mut R::Cursor) {}
}

impl List for () {}
impl Unmount for () {
    fn unmount<R: Renderer>(&mut self) {}
}
