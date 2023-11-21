use crate::{Attr, Diff, Platform, Unmount, View, ViewState};

impl<T: Diff> Diff for Option<T>
where
    T::State: Unmount,
{
    type State = State<T>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        State(self.map(|value| value.init::<P>(cursor)))
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        match (state, self) {
            (State(Some(state)), Some(value)) => {
                value.diff::<P>(state, cursor);
            }
            (State(Some(state)), None) => {
                state.unmount::<P>(cursor);
            }
            (state @ State(None), Some(value)) => {
                *state = State(Some(value.init::<P>(cursor)));
            }
            (State(None), None) => {}
        }
    }
}

pub struct State<T: Diff>(Option<T::State>);

impl<T: Diff> Unmount for State<T>
where
    T::State: Unmount,
{
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        if let Some(state) = &mut self.0 {
            <T::State as Unmount>::unmount::<P>(state, cursor);
        }
    }
}

impl<T: View> ViewState for State<T> where Self: Unmount {}
impl<T: Attr> Attr for Option<T> where T::State: Unmount {}
