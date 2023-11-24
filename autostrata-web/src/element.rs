use autostrata::{platform::Cursor, AttrSet, Children, Diff, View};

use crate::{Web, WebCursor};

pub struct Element<A, C> {
    name: &'static str,
    attrs: A,
    children: C,
}

impl<A, C> Element<A, C> {
    pub const fn new(name: &'static str, attrs: A, children: C) -> Self {
        Self {
            name,
            attrs,
            children,
        }
    }
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
