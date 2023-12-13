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
    type ConstState = Either<L::ConstState, R::ConstState>;
    type DiffState = Either<L::DiffState, R::DiffState>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        match self {
            Self::Left(left) => Either::Left(left.init_const(cursor)),
            Self::Right(right) => Either::Right(right.init_const(cursor)),
        }
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        match self {
            Self::Left(left) => Either::Left(left.init_diff(cursor)),
            Self::Right(right) => Either::Right(right.init_diff(cursor)),
        }
    }

    fn diff(self, mut state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        match (&mut state, self) {
            (Either::Left(left_state), Either::Left(left)) => {
                left.diff(left_state, cursor);
            }
            (Either::Right(right_state), Either::Right(right)) => {
                right.diff(right_state, cursor);
            }
            (Either::Left(_), Either::Right(right)) => cursor.replace(|cursor| {
                *state = Either::Right(right.init_diff(cursor));
            }),
            (Either::Right(_), Either::Left(left)) => cursor.replace(|cursor| {
                *state = Either::Left(left.init_diff(cursor));
            }),
        }
    }
}
