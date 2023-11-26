use std::fmt::Display;
use std::fmt::Write;

use crate::{platform::Cursor, Diff, ElementHandle, Platform, View};

/// Literal text.
#[derive(Clone, Copy)]
pub struct Text(pub &'static str);

impl<P: Platform> Diff<P> for Text {
    type State = (ElementHandle, &'static str);

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        (cursor.text(self.0), self.0)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self.0 != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(self.0);
            *old = self.0;
        }
    }
}

impl<P: Platform> View<P> for Text {}

/// Things that can be formatted _into_ text.
#[derive(Clone, Copy)]
pub struct Format<T>(pub T);

impl<P: Platform, T: Display + 'static> Diff<P> for Format<T> {
    type State = (ElementHandle, String);

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        (cursor.text(&string), string)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        P::log(&format!("Format diff new=`{string}` old=`{old}`"));

        if string != *old {
            let mut cursor = P::Cursor::from_element_handle(&handle);
            cursor.update_text(&string);
            *old = string;
        }
    }
}

impl<P: Platform, T: Display + 'static> View<P> for Format<T> {}
