use crate::{prelude::Platform, Diff, View};

pub struct Iter<I>(pub I);

impl<P: Platform, T: Diff<P>, I: Iterator<Item = T>> Diff<P> for Iter<I> {
    type State = Vec<T::State>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        let mut state = Vec::with_capacity(self.0.size_hint().1.unwrap_or(0));

        for item in self.0 {
            state.push(item.init(cursor));
        }

        state
    }

    fn diff(self, _state: &mut Self::State, _cursor: &mut <P as Platform>::Cursor) {
        todo!()
    }
}

impl<P: Platform, T: View<P>, I: Iterator<Item = T>> View<P> for Iter<I> {}
