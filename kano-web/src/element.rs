use kano::{Children, Diff, Props, View};

use crate::{props::HtmlProperties, Web, WebCursor};

#[derive(Clone, Copy)]
pub struct Element<T, C> {
    name: &'static str,
    props: T,
    children: C,
}

impl<T: Props<HtmlProperties> + Diff<Web>, C> Element<T, C> {
    pub const fn new(name: &'static str, props: T, children: C) -> Self {
        Self {
            name,
            props,
            children,
        }
    }
}

impl<T: Props<HtmlProperties> + Diff<Web>, C: Children<Web>> Diff<Web> for Element<T, C> {
    type State = State<T, C>;

    fn init(self, cursor: &mut WebCursor) -> Self::State {
        let _ = cursor.element(self.name);
        let props = self.props.init(cursor);
        let children = self.children.init(cursor);

        State { props, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut crate::WebCursor) {
        self.props.diff(&mut state.props, cursor);
        self.children.diff(&mut state.children, cursor);
    }
}

impl<T: Props<HtmlProperties> + Diff<Web>, C: Children<Web>> View<Web> for Element<T, C> {}

pub struct State<T: Diff<Web>, C: Children<Web>> {
    props: T::State,
    children: C::State,
}
