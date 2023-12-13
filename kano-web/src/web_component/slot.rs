use kano::{markup::Cursor, Children};

use crate::{web_cursor::WebCursor, Html5, Web};

/// Special case Children for web components.
/// The slot's inner contents is outside Kano's control.
pub struct Slot;

impl Children<Web, Html5> for Slot {
    type ConstState = ();
    type DiffState = ();

    fn init_const(self, cursor: &mut WebCursor) -> Self::ConstState {
        <WebCursor as Cursor>::enter_children(cursor);
        cursor.element("slot");
        <WebCursor as Cursor>::exit_children(cursor);
    }

    fn init_diff(self, cursor: &mut WebCursor) -> Self::DiffState {
        self.init_const(cursor);
    }

    fn diff(self, _state: &mut Self::DiffState, _cursor: &mut WebCursor) {}
}
