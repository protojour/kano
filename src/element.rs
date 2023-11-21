use crate::{Diff, Handle, List, Renderer, Unmount};

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

impl<A: Diff, C: Diff> Diff for Element<A, C> {
    type State = ElementState<A, C>;

    fn init<R: Renderer>(self, cursor: &mut R::Cursor) -> Self::State {
        let handle = R::new_element(cursor, self.name);

        R::enter_attrs(cursor);
        let attrs = self.attrs.init::<R>(cursor);
        R::exit_attrs(cursor);

        let children = self.children.init::<R>(cursor);

        ElementState {
            handle,
            attrs,
            children,
        }
    }

    fn diff<R: Renderer>(self, state: &mut Self::State, cursor: &mut R::Cursor) {
        R::enter_attrs(cursor);
        self.attrs.diff::<R>(&mut state.attrs, cursor);
        R::exit_attrs(cursor);

        self.children.diff::<R>(&mut state.children, cursor);
    }
}

pub struct ElementState<A: Diff, C: Diff> {
    handle: Handle,
    attrs: <A as Diff>::State,
    children: <C as Diff>::State,
}

impl<A: Diff, C: Diff> Unmount for ElementState<A, C> {
    fn unmount<R: Renderer>(&mut self) {
        R::unmount(&mut self.handle);
    }
}
