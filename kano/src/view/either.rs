//! Just a test for reactivity

use crate::{
    markup::{Cursor, Markup},
    View,
};

#[derive(Clone, Copy)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<P, M, L, R> View<P, M> for Either<L, R>
where
    M: Markup<P>,
    L: View<P, M>,
    R: View<P, M>,
{
    type State = State<P, M, L, R>;

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        match self {
            Self::Left(left) => State {
                state: Either::Left(left.init(cursor)),
            },
            Self::Right(right) => State {
                state: Either::Right(right.init(cursor)),
            },
        }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut M::Cursor) {
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

pub struct State<P, M: Markup<P>, L: View<P, M>, R: View<P, M>> {
    state: Either<L::State, R::State>,
}
