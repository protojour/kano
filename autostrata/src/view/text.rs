use crate::{platform::Cursor, Diff, ElementHandle, Platform, View};

impl<P: Platform> Diff<P> for &'static str {
    type State = (ElementHandle, Self);

    fn init(self, cursor: &mut P::Cursor) -> (ElementHandle, Self) {
        (cursor.text(self), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(&self);
            *old = self;
        }
    }
}

impl<P: Platform> Diff<P> for String {
    type State = (ElementHandle, Self);

    fn init(self, cursor: &mut P::Cursor) -> (ElementHandle, Self) {
        (cursor.text(self.as_str()), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(&self);
            *old = self;
        }
    }
}

impl<P: Platform> View<P> for &'static str {}
impl<P: Platform> View<P> for String {}
