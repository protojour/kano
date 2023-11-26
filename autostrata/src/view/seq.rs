use std::ops::Deref;

use crate::{
    prelude::{Platform, Ref},
    Diff, View,
};

pub struct Seq<S>(pub S);

pub trait Map<F>: Sized {
    type Seq;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F>;
}

impl<M, T, F: Fn(M) -> T> Map<F> for Seq<Vec<M>> {
    type Seq = Vec<M>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self.0, func)
    }
}

impl<M, T, F: Fn(M) -> T> Map<F> for Seq<Ref<Vec<M>>> {
    type Seq = Ref<Vec<M>>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self.0, func)
    }
}

pub struct SeqMap<S, F>(S, F);

impl<P: Platform, M, T: Diff<P>, F: Fn(M) -> T> Diff<P> for SeqMap<Vec<M>, F> {
    type State = Vec<T::State>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        let mut state = Vec::with_capacity(self.0.len());

        for item in self.0 {
            state.push((self.1)(item).init(cursor));
        }

        state
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut <P as Platform>::Cursor) {
        todo!()
    }
}

impl<P: Platform, M: Clone + 'static, T: View<P>, F: Fn(M) -> T> View<P> for SeqMap<Vec<M>, F> {}

impl<P: Platform, M: Clone + 'static, T: Diff<P>, F: Fn(M) -> T> Diff<P>
    for SeqMap<Ref<Vec<M>>, F>
{
    type State = Vec<T::State>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        let borrow = self.0.borrow();
        let vec = borrow.deref();

        let mut state = Vec::with_capacity(vec.len());

        for item in vec {
            state.push((self.1)(item.clone()).init(cursor));
        }

        state
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut <P as Platform>::Cursor) {
        todo!()
    }
}

impl<P: Platform, M: Clone + 'static, T: View<P>, F: Fn(M) -> T> View<P>
    for SeqMap<Ref<Vec<M>>, F>
{
}
