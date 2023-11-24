use crate::{platform::Cursor, Diff, ElementHandle, Platform, View};

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

impl View for &'static str {}
impl View for String {}
