use kano::{platform::Cursor, Children, Diff};

use crate::{web_cursor::WebCursor, Web};

/// Special case Children for web components.
/// The slot's inner contents is outside Kano's control.
pub struct Slot;

impl Diff<Web> for Slot {
    type State = ();

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        cursor.enter_children();
        cursor.element("slot");
        cursor.exit_children();
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut WebCursor) {}
}

impl Children<Web> for Slot {}
