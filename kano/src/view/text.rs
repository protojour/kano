use std::fmt::Display;
use std::fmt::Write;

use crate::markup::{Cursor, Markup};
use crate::View;

impl<P, M: Markup<P>> View<P, M> for &'static str {
    type State = (<M::Cursor as Cursor>::TextHandle, &'static str);

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        (cursor.text(self), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut M::Cursor) {
        if self != *old {
            let mut cursor = M::Cursor::from_text_handle(handle);
            cursor.update_text(self);
            *old = self;
        }
    }
}

impl<P, M: Markup<P>> View<P, M> for String {
    type State = (<M::Cursor as Cursor>::TextHandle, String);

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        (cursor.text(self.as_str()), self)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut M::Cursor) {
        if self != *old {
            let mut cursor = M::Cursor::from_text_handle(handle);
            cursor.update_text(self.as_str());
            *old = self;
        }
    }
}

/// Things that can be formatted _into_ text.
#[derive(Clone, Copy)]
pub struct Fmt<T>(pub T);

impl<P, M: Markup<P>, T: Display + 'static> View<P, M> for Fmt<T> {
    type State = (<M::Cursor as Cursor>::TextHandle, String);

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        (cursor.text(&string), string)
    }

    fn diff(self, (handle, old): &mut Self::State, _cursor: &mut M::Cursor) {
        let mut string = String::new();
        write!(&mut string, "{}", self.0).unwrap();

        crate::log(&format!("Format diff new=`{string}` old=`{old}`"));

        if string != *old {
            let mut cursor = M::Cursor::from_text_handle(handle);
            cursor.update_text(&string);
            *old = string;
        }
    }
}
