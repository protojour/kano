//! Just a test for reactivity

use crate::{platform::Platform, Attr, Diff, Unmount, View, ViewState};

#[derive(Clone, Copy)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Diff, R: Diff> Diff for Either<L, R>
where
    L::State: Unmount,
    R::State: Unmount,
{
    type State = State<L, R>;

    fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
        match self {
            Self::Left(left) => State {
                state: Either::Left(left.init::<P>(cursor)),
            },
            Self::Right(right) => State {
                state: Either::Right(right.init::<P>(cursor)),
            },
        }
    }

    fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        match (&mut state.state, self) {
            (Either::Left(left_state), Either::Left(left)) => {
                left.diff::<P>(left_state, cursor);
            }
            (Either::Right(right_state), Either::Right(right)) => {
                right.diff::<P>(right_state, cursor);
            }
            (Either::Left(left_state), Either::Right(right)) => {
                left_state.unmount::<P>(cursor);
                state.state = Either::Right(right.init::<P>(cursor));
            }
            (Either::Right(right_state), Either::Left(left)) => {
                right_state.unmount::<P>(cursor);
                state.state = Either::Left(left.init::<P>(cursor));
            }
        }
    }
}

pub struct State<L: Diff, R: Diff> {
    state: Either<L::State, R::State>,
}

impl<L: Diff, R: Diff> Unmount for State<L, R>
where
    L::State: Unmount,
    R::State: Unmount,
{
    fn unmount<P: Platform>(&mut self, cursor: &mut P::Cursor) {
        match &mut self.state {
            Either::Left(left) => {
                <L::State as Unmount>::unmount::<P>(left, cursor);
            }
            Either::Right(right) => {
                <R::State as Unmount>::unmount::<P>(right, cursor);
            }
        }
    }
}

impl<L: View, R: View> ViewState for State<L, R> where Self: Unmount {}
impl<L: Attr, R: Attr> Attr for Either<L, R>
where
    L::State: Unmount,
    R::State: Unmount,
{
}
