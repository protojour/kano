use std::{marker::PhantomData, ops::Deref};

use crate::{
    log,
    platform::Cursor,
    prelude::{Platform, Ref},
    Diff, View,
};

pub trait Map<F>: Sized {
    type Seq;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F>;
}

impl<M, T, F: Fn(M) -> T> Map<F> for Vec<M> {
    type Seq = Vec<M>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self, func)
    }
}

impl<M, T, F: Fn(M) -> T> Map<F> for Ref<Vec<M>> {
    type Seq = Ref<Vec<M>>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self, func)
    }
}

pub fn seq_map<F, M: Map<F>>(seq: M, func: F) -> SeqMap<<M as Map<F>>::Seq, F> {
    seq.seq_map(func)
}

pub struct SeqMap<S, F>(S, F);

impl<P: Platform, M, T: Diff<P>, F: Fn(M) -> T> Diff<P> for SeqMap<Vec<M>, F> {
    type State = Vec<T::State>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        let SeqMap(vec, func) = self;
        let mut state = Vec::with_capacity(vec.len());

        for model_elem in vec {
            state.push(func(model_elem).init(cursor));
        }

        state
    }

    fn diff(self, state: &mut Self::State, cursor: &mut <P as Platform>::Cursor) {
        let SeqMap(model, func) = self;

        Differ::<P>::apply_diff(
            model.len(),
            model.into_iter(),
            state,
            |model_element| func(model_element),
            cursor,
        );
    }
}

impl<P: Platform, M: Clone + 'static, T: View<P>, F: Fn(M) -> T> View<P> for SeqMap<Vec<M>, F> {}

impl<P: Platform, M: Clone + 'static, T: Diff<P>, F: Fn(M) -> T> Diff<P>
    for SeqMap<Ref<Vec<M>>, F>
{
    type State = Vec<T::State>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        let model_borrow = self.0.borrow();
        let model = model_borrow.deref();

        let mut state = Vec::with_capacity(model.len());

        for model_elem in model {
            state.push((self.1)(model_elem.clone()).init(cursor));
        }

        state
    }

    fn diff(self, state: &mut Self::State, cursor: &mut <P as Platform>::Cursor) {
        let SeqMap(model, func) = self;
        let model = model.borrow();

        Differ::<P>::apply_diff(
            model.len(),
            model.iter(),
            state,
            |model_element| func(model_element.clone()),
            cursor,
        );
    }
}

impl<P: Platform, M: Clone + 'static, T: View<P>, F: Fn(M) -> T> View<P>
    for SeqMap<Ref<Vec<M>>, F>
{
}

struct Differ<P>(PhantomData<P>);

impl<P: Platform> Differ<P> {
    fn apply_diff<T: Diff<P>, M, MI: Iterator<Item = M>, F: Fn(M) -> T>(
        model_len: usize,
        model_iter: MI,
        state: &mut Vec<T::State>,
        func: F,
        cursor: &mut <P as Platform>::Cursor,
    ) {
        log(&format!(
            "apply_diff model.len = {model_len} state.len = {}",
            state.len()
        ));

        let mut model_iter = model_iter.peekable();
        let mut state_iter = state.iter_mut().peekable();

        while model_iter.peek().is_some() && state_iter.peek().is_some() {
            let item = func(model_iter.next().unwrap());
            item.diff(state_iter.next().unwrap(), cursor);
            cursor.next_sibling();
        }

        // Delete elements
        while let Some(_) = state_iter.next() {
            cursor.remove();
        }

        // Append new items
        while let Some(model_elem) = model_iter.next() {
            log(&format!("Appending"));
            state.push(func(model_elem).init(cursor));
        }

        state.truncate(model_len);
    }
}
