use crate::{Diff, Handle, Platform, ViewState};

impl Diff for &'static str {
    type State = (Handle, Self);

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> (Handle, Self) {
        (P::new_text(self, cursor), self)
    }

    fn diff<P: Platform>(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            P::update_text(handle, &self);
            *old = self;
        }
    }
}

impl ViewState for (Handle, &'static str) {}
