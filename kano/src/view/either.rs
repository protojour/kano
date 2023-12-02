//! Just a test for reactivity

use crate::{
    platform::{Cursor, Platform},
    Diff, View,
};

#[derive(Clone, Copy)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<P: Platform, L: Diff<P>, R: Diff<P>> Diff<P> for Either<L, R> {
    type State = State<P, L, R>;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        match self {
            Self::Left(left) => State {
                state: Either::Left(left.init(cursor)),
            },
            Self::Right(right) => State {
                state: Either::Right(right.init(cursor)),
            },
        }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
        match (&mut state.state, self) {
            (Either::Left(left_state), Either::Left(left)) => {
                left.diff(left_state, cursor);
            }
            (Either::Right(right_state), Either::Right(right)) => {
                right.diff(right_state, cursor);
            }
            (Either::Left(_), Either::Right(right)) => cursor.replace(|cursor| {
                state.state = Either::Right(right.init(cursor));
            }),
            (Either::Right(_), Either::Left(left)) => cursor.replace(|cursor| {
                state.state = Either::Left(left.init(cursor));
            }),
        }
    }
}

pub struct State<P: Platform, L: Diff<P>, R: Diff<P>> {
    state: Either<L::State, R::State>,
}

impl<P: Platform, L: View<P>, R: View<P>> View<P> for Either<L, R> {}
