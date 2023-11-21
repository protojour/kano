use crate::{Diff, Platform, Unmount};

impl<T: Diff> Diff for Option<T>
where
    T::State: Unmount,
{
    type State = Option<T::State>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        self.map(|value| value.init::<P>(cursor))
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        match (state, self) {
            (Some(state), Some(value)) => {
                value.diff::<P>(state, cursor);
            }
            (Some(state), None) => {
                state.unmount::<P>(cursor);
            }
            (state @ None, Some(value)) => {
                *state = Some(value.init::<P>(cursor));
            }
            (None, None) => {}
        }
    }
}
