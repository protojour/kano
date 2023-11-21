use crate::{Diff, Handle, Renderer};

impl Diff for &'static str {
    type State = (Handle, Self);

    fn init<R: Renderer>(self, cursor: &mut R::Cursor) -> (Handle, Self) {
        (R::new_text(self, cursor), self)
    }

    fn diff<R: Renderer>(self, (handle, old): &mut Self::State, _cursor: &mut R::Cursor) {
        if self != *old {
            R::update_text(handle, &self);
            *old = self;
        }
    }
}
