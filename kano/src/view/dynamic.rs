use std::any::Any;

use crate::{platform::Cursor, prelude::platform::Platform, View};

/// A type-erased View.
pub struct Dyn<P> {
    inner: Box<dyn DynView<P>>,
}

impl<P: Platform> Dyn<P> {
    pub fn new<T: View<P> + 'static>(diff: T) -> Self {
        Self {
            inner: Box::new(DynWrapper(Some(diff))),
        }
    }
}

trait DynView<P: Platform> {
    fn init(&mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static>;
    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor);
}

impl<P: Platform> View<P> for Dyn<P> {
    type State = Box<dyn Any + 'static>;

    fn init(mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static> {
        self.inner.init(cursor)
    }

    fn diff(mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor) {
        self.inner.diff(state, cursor);
    }
}

struct DynWrapper<T>(Option<T>);

impl<P: Platform, T: View<P> + View<P>> DynView<P> for DynWrapper<T>
where
    <T as View<P>>::State: 'static,
{
    fn init(&mut self, cursor: &mut <P as Platform>::Cursor) -> Box<dyn Any + 'static> {
        let inner = self.0.take().unwrap();
        Box::new(inner.init(cursor))
    }

    fn diff(&mut self, state: &mut Box<dyn Any + 'static>, cursor: &mut <P as Platform>::Cursor) {
        let inner = self.0.take().unwrap();
        if let Some(state) = state.downcast_mut::<T::State>() {
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

    fn _comp() -> impl View<TestPlatform> {
        Dyn::new(view! {
            ""
        })
    }
}
