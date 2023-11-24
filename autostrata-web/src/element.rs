use autostrata::{platform::Cursor, AttrSet, Children, Diff, View};

use crate::{Web, WebCursor};

pub struct Element<A, C> {
    pub(crate) name: &'static str,
    pub(crate) attrs: A,
    pub(crate) children: C,
}

impl<A: AttrSet<Web>, C: Children<Web>> Diff<Web> for Element<A, C> {
    type State = State<A, C>;

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        let _ = cursor.element(self.name);

        cursor.enter_attrs();
        let attrs = self.attrs.init(cursor);
        cursor.exit_attrs();

        let children = self.children.init(cursor);

        State { attrs, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut crate::WebCursor) {
        cursor.enter_attrs();
        self.attrs.diff(&mut state.attrs, cursor);
        cursor.exit_attrs();

        self.children.diff(&mut state.children, cursor);
    }
}

impl<A: AttrSet<Web>, C: Children<Web>> View<Web> for Element<A, C> {}

pub struct State<A: AttrSet<Web>, C: Children<Web>> {
    attrs: A::State,
    children: C::State,
}
