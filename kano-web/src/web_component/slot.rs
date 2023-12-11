use kano::{markup::Cursor, Children};

use crate::{web_cursor::WebCursor, Html5, Web};

/// Special case Children for web components.
/// The slot's inner contents is outside Kano's control.
pub struct Slot;

impl Children<Web, Html5> for Slot {
    type State = ();

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        <WebCursor as Cursor>::enter_children(cursor);
        cursor.element("slot");
        <WebCursor as Cursor>::exit_children(cursor);
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut WebCursor) {}
}
