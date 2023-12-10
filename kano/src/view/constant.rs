use crate::{prelude::platform::Platform, View};

/// A constant view, that is never supposed to change internally once rendered.
pub struct Const<V>(pub V);

impl<P: Platform, V: View<P>> View<P> for Const<V> {
    type State = V::State;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        // TODO: Add cursor API to tell the platform that a constant subtree is being entered,
        // so the platform can solve this in the most performant way.
        V::init(self.0, cursor)
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut <P as Platform>::Cursor) {
        // The whole point of Const is to skip the diff!
    }
}
