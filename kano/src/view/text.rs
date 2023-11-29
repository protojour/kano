use std::fmt::Display;
use std::fmt::Write;

use crate::{platform::Cursor, Diff, Platform, View};

impl<P: Platform> Diff<P> for &'static str {
    type State = (<P::Cursor as Cursor>::TextHandle, &'static str);

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        (cursor.text(self), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_text_handle(handle);
            cursor.update_text(self);
            *old = self;
        }
    }
}

impl<P: Platform> View<P> for &'static str {}

impl<P: Platform> Diff<P> for String {
    type State = (<P::Cursor as Cursor>::TextHandle, String);

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        (cursor.text(self.as_str()), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        if self != *old {
            let mut cursor = P::Cursor::from_text_handle(handle);
            cursor.update_text(self.as_str());
            *old = self;
        }
    }
}

impl<P: Platform> View<P> for String {}

/// Things that can be formatted _into_ text.
#[derive(Clone, Copy)]
pub struct Fmt<T>(pub T);

impl<P: Platform, T: Display + 'static> Diff<P> for Fmt<T> {
    type State = (<P::Cursor as Cursor>::TextHandle, String);

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        (cursor.text(&string), string)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut P::Cursor) {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        crate::log(&format!("Format diff new=`{string}` old=`{old}`"));

        if string != *old {
            let mut cursor = P::Cursor::from_text_handle(handle);
            cursor.update_text(&string);
            *old = string;
        }
    }
}

impl<P: Platform, T: Display + 'static> View<P> for Fmt<T> {}
