use crate::{platform::Cursor, Diff, ElementHandle, Platform, ViewState};

impl Diff for &'static str {
    type State = (ElementHandle, Self);

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> (ElementHandle, Self) {
        (cursor.text(self), self)
    }

    fn diff<P: Platform>(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(&self);
            *old = self;
        }
    }
}

impl ViewState for (ElementHandle, &'static str) {}

impl Diff for String {
    type State = (ElementHandle, Self);

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> (ElementHandle, Self) {
        (cursor.text(self.as_str()), self)
    }

    fn diff<P: Platform>(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(&self);
            *old = self;
        }
    }
}

impl ViewState for (ElementHandle, String) {}
