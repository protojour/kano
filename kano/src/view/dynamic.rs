use std::any::Any;

use crate::{
    markup::{Cursor, Markup},
    View,
};

/// A type-erased View.
pub struct Dyn<P, M> {
    inner: Box<dyn DynView<P, M>>,
}

impl<P: 'static, M: Markup<P>> Dyn<P, M> {
    pub fn new<T: View<P, M> + 'static>(diff: T) -> Self {
        Self {
            inner: Box::new(DynWrapper(Some(diff))),
        }
    }
}

trait DynView<P, M: Markup<P>> {
    fn init(&mut self, cursor: &mut M::Cursor) -> Box<dyn Any + 'static>;
    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut M::Cursor);
}

impl<P, M: Markup<P>> View<P, M> for Dyn<P, M> {
    type State = Box<dyn Any + 'static>;

    fn init(mut self, cursor: &mut M::Cursor) -> Box<dyn Any + 'static> {
        self.inner.init(cursor)
    }

    fn diff(mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut M::Cursor) {
        self.inner.diff(state, cursor);
    }
}

struct DynWrapper<T>(Option<T>);

impl<P, M, V> DynView<P, M> for DynWrapper<V>
where
    M: Markup<P>,
    V: View<P, M>,
    <V as View<P, M>>::State: 'static,
{
    fn init(&mut self, cursor: &mut M::Cursor) -> Box<dyn Any + 'static> {
        let inner = self.0.take().unwrap();
        Box::new(inner.init(cursor))
    }

    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut M::Cursor) {
        let inner = self.0.take().unwrap();
        if let Some(state) = state.downcast_mut::<V::State>() {
            inner.diff(state, cursor);
        } else {
            cursor.replace(|cursor| {
                *state = Box::new(inner.init(cursor));
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{platform::test_platform::TestPlatform, view, View};

    use super::Dyn;

    fn _comp() -> impl View<TestPlatform, ()> {
        Dyn::new(view! {
            ""
        })
    }
}
