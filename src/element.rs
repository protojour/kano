use crate::{AttrSet, Diff, Handle, List, Platform, Unmount};

pub struct Element<A, C> {
    name: &'static str,
    attrs: A,
    children: C,
}

impl<A: List, C: List> Element<A, C> {
    pub fn new(name: &'static str, attrs: A, children: C) -> Self {
        Self {
            name,
            attrs,
            children,
        }
    }
}

impl<A: AttrSet, C: List> Diff for Element<A, C> {
    type State = ElementState<A, C>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        let handle = P::new_element(cursor, self.name);

        P::enter_attrs(cursor);
        let attrs = self.attrs.init::<P>(cursor);
        P::exit_attrs(cursor);

        let children = self.children.init::<P>(cursor);

        ElementState {
            handle,
            attrs,
            children,
        }
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        P::enter_attrs(cursor);
        self.attrs.diff::<P>(&mut state.attrs, cursor);
        P::exit_attrs(cursor);

        self.children.diff::<P>(&mut state.children, cursor);
    }
}

pub struct ElementState<A: AttrSet, C: List> {
    handle: Handle,
    attrs: <A as Diff>::State,
    children: <C as Diff>::State,
}

impl<A: AttrSet, C: List> Unmount for ElementState<A, C> {
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.handle, cursor);
    }
}
