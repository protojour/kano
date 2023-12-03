use std::any::Any;

use crate::{prelude::platform::Platform, Diff, View};

/// A type-erased View.
pub struct Dyn<P> {
    inner: Box<dyn DynView<P>>,
}

impl<P: Platform> Dyn<P> {
    pub fn new<T: Diff<P> + View<P> + 'static>(diff: T) -> Self {
        Self {
            inner: Box::new(DynWrapper(Some(diff))),
        }
    }
}

trait DynView<P: Platform> {
    fn init(&mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static>;
    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor);
}

impl<P: Platform> Diff<P> for Dyn<P> {
    type State = Box<dyn Any + 'static>;

    fn init(mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static> {
        self.inner.init(cursor)
    }

    fn diff(mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor) {
        self.inner.diff(state, cursor);
    }
}

impl<P: Platform> View<P> for Dyn<P> {}

struct DynWrapper<T>(Option<T>);

impl<P: Platform, T: Diff<P> + View<P>> DynView<P> for DynWrapper<T>
where
    <T as Diff<P>>::State: 'static,
{
    fn init(&mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static> {
        let inner = self.0.take().unwrap();
        Box::new(inner.init(cursor))
    }

    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor) {
        let inner = self.0.take().unwrap();
        let state = state.downcast_mut::<T::State>().unwrap();
        inner.diff(state, cursor);
    }
}

#[cfg(test)]
mod tests {
    use crate::{platform::test_platform::TestPlatform, view, View};

    use super::Dyn;

    fn _comp() -> impl View<TestPlatform> {
        Dyn::new(view! {
            ""
        })
    }
}
