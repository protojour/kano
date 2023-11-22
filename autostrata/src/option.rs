use crate::{Attr, Diff, Platform, View, ViewState};

impl<T: Diff> Diff for Option<T> {
    type State = State<T>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        State(self.map(|value| value.init::<P>(cursor)))
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        match (state, self) {
            (State(Some(state)), Some(value)) => {
                value.diff::<P>(state, cursor);
            }
            (state @ State(Some(_)), None) => {
                P::replace_at_cursor(cursor, |_| {});
                *state = State(None);
            }
            (state @ State(None), Some(value)) => {
                *state = State(Some(value.init::<P>(cursor)));
            }
            (State(None), None) => {}
        }
    }
}

pub struct State<T: Diff>(Option<T::State>);

impl<T: View> ViewState for State<T> {}
impl<T: Attr> Attr for Option<T> {}
