use std::{marker::PhantomData, ops::Deref};

use crate::{log, markup::Cursor, markup::Markup, reactive::Ref, View};

pub trait Map<F>: Sized {
    type Seq;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F>;
}

impl<T, V, F: Fn(T) -> V> Map<F> for Vec<T> {
    type Seq = Vec<T>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self, func)
    }
}

impl<T, V, F: Fn(T) -> V> Map<F> for Ref<Vec<T>> {
    type Seq = Ref<Vec<T>>;

    fn seq_map(self, func: F) -> SeqMap<Self::Seq, F> {
        SeqMap(self, func)
    }
}

pub fn seq_map<F, S: Map<F>>(seq: S, func: F) -> SeqMap<<S as Map<F>>::Seq, F> {
    seq.seq_map(func)
}

pub struct SeqMap<S, F>(S, F);

impl<P, M, V, T, F> View<P, M> for SeqMap<Vec<T>, F>
where
    M: Markup<P>,
    V: View<P, M>,
    F: Fn(T) -> V,
{
    type ConstState = Vec<V::ConstState>;
    type DiffState = Vec<V::DiffState>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        let SeqMap(vec, func) = self;
        let mut state = Vec::with_capacity(vec.len());

        for model_elem in vec {
            state.push(func(model_elem).init_const(cursor));
        }

        state
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        let SeqMap(vec, func) = self;
        let mut state = Vec::with_capacity(vec.len());

        for model_elem in vec {
            state.push(func(model_elem).init_diff(cursor));
        }

        state
    }

    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        let SeqMap(model, func) = self;

        Differ::<P, M>::apply_diff(model.len(), model.into_iter(), state, func, cursor);
    }
}

impl<P, M, V, T, F> View<P, M> for SeqMap<Ref<Vec<T>>, F>
where
    M: Markup<P>,
    V: View<P, M>,
    T: Clone + 'static,
    F: Fn(T) -> V,
{
    type ConstState = Vec<V::ConstState>;
    type DiffState = Vec<V::DiffState>;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        let model_borrow = self.0.borrow();
        let model = model_borrow.deref();

        let mut state = Vec::with_capacity(model.len());

        for model_elem in model {
            state.push((self.1)(model_elem.clone()).init_const(cursor));
        }

        state
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        let model_borrow = self.0.borrow();
        let model = model_borrow.deref();

        let mut state = Vec::with_capacity(model.len());

        for model_elem in model {
            state.push((self.1)(model_elem.clone()).init_diff(cursor));
        }

        state
    }

    fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
        let SeqMap(model, func) = self;
        let model = model.borrow();

        Differ::<P, M>::apply_diff(
            model.len(),
            model.iter(),
            state,
            |model_element| func(model_element.clone()),
            cursor,
        );
    }
}

struct Differ<P, S>(PhantomData<P>, PhantomData<S>);

impl<P, M: Markup<P>> Differ<P, M> {
    fn apply_diff<V, T, TI, F>(
        model_len: usize,
        model_iter: TI,
        state: &mut Vec<V::DiffState>,
        func: F,
        cursor: &mut M::Cursor,
    ) where
        V: View<P, M>,
        TI: Iterator<Item = T>,
        F: Fn(T) -> V,
    {
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
        for _ in state_iter {
            cursor.remove();
        }

        // Append new items
        for model_elem in model_iter {
            log("Appending");
            state.push(func(model_elem).init_diff(cursor));
        }

        state.truncate(model_len);
    }
}
