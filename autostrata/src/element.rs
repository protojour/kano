use crate::{
    platform::{Handle, Platform},
    AttrSet, Children, Diff, Unmount, ViewState,
};

pub struct Element<A, C> {
    name: &'static str,
    attrs: A,
    children: C,
}

impl<A: AttrSet, C: Children> Element<A, C> {
    pub fn new(name: &'static str, attrs: A, children: C) -> Self {
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
        let handle = P::new_element(self.name, cursor);

        P::enter_attrs(cursor);
        let attrs = self.attrs.init::<P>(cursor);
        P::exit_attrs(cursor);

        let children = self.children.init::<P>(cursor);

        State {
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

pub struct State<A: AttrSet, C: Children> {
    handle: Handle,
    attrs: A::State,
    children: C::State,
}

impl<A: AttrSet, C: Children> Unmount for State<A, C> {
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        P::unmount(&mut self.handle, cursor);
    }
}

impl<A: AttrSet, C: Children> ViewState for State<A, C> {}
