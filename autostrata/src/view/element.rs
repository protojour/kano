use crate::{
    platform::{Cursor, Platform},
    AttrSet, Children, Diff, View,
};

pub struct Element<A, C> {
    name: &'static str,
    attrs: A,
    children: C,
}

impl<A: AttrSet, C: Children> Element<A, C> {
    pub const fn new(name: &'static str, attrs: A, children: C) -> Self {
        Self {
            name,
            attrs,
            children,
        }
    }
}

impl<A: AttrSet, C: Children> Diff for Element<A, C> {
    type State = State<A, C>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let _ = cursor.element(self.name);

        cursor.enter_attrs();
        let attrs = self.attrs.init::<P>(cursor);
        cursor.exit_attrs();

        let children = self.children.init::<P>(cursor);

        State { attrs, children }
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        cursor.enter_attrs();
        self.attrs.diff::<P>(&mut state.attrs, cursor);
        cursor.exit_attrs();

        self.children.diff::<P>(&mut state.children, cursor);
    }
}

impl<A: AttrSet, C: Children> View for Element<A, C> {}

pub struct State<A: AttrSet, C: Children> {
    attrs: A::State,
    children: C::State,
}
