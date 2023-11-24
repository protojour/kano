use std::marker::PhantomData;

use crate::{
    platform::{Cursor, Platform},
    AttrSet, Children, Diff, View,
};

pub struct Element<P, A, C> {
    platform: PhantomData<P>,
    name: &'static str,
    attrs: A,
    children: C,
}

impl<P: Platform, A: AttrSet<P>, C: Children<P>> Element<P, A, C> {
    pub const fn new(name: &'static str, attrs: A, children: C) -> Self {
        Self {
            platform: PhantomData,
            name,
            attrs,
            children,
        }
    }
}

impl<P: Platform, A: AttrSet<P>, C: Children<P>> Diff<P> for Element<P, A, C> {
    type State = State<P, A, C>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        let _ = cursor.element(self.name);

        cursor.enter_attrs();
        let attrs = self.attrs.init(cursor);
        cursor.exit_attrs();

        let children = self.children.init(cursor);

        State { attrs, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        cursor.enter_attrs();
        self.attrs.diff(&mut state.attrs, cursor);
        cursor.exit_attrs();

        self.children.diff(&mut state.children, cursor);
    }
}

impl<P: Platform, A: AttrSet<P>, C: Children<P>> View<P> for Element<P, A, C> {}

pub struct State<P: Platform, A: AttrSet<P>, C: Children<P>> {
    attrs: A::State,
    children: C::State,
}
