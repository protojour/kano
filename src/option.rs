use crate::{Diff, Unmount};

impl<T: Diff> Diff for Option<T>
where
    T::State: Unmount,
{
    type State = Option<T::State>;

    fn init<R: crate::Renderer>(self, cursor: &mut R::Cursor) -> Self::State {
        self.map(|value| value.init::<R>(cursor))
    }

    fn diff<R: crate::Renderer>(self, state: &mut Self::State, cursor: &mut R::Cursor) {
        match (state, self) {
            (Some(state), Some(value)) => {
                value.diff::<R>(state, cursor);
            }
            (Some(state), None) => {
                state.unmount::<R>();
            }
            (state @ None, Some(value)) => {
                *state = Some(value.init::<R>(cursor));
            }
            (None, None) => {}
        }
    }
}
