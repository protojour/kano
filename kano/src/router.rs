use crate::{prelude::platform::Platform, view::Dyn, View};

pub struct Router<P: Platform> {
    inner: matchit::Router<Box<dyn Fn() -> Dyn<P>>>,
    fallback_fn: Box<dyn Fn() -> Dyn<P>>,
}

pub struct Builder<P: Platform> {
    inner: matchit::Router<Box<dyn Fn() -> Dyn<P>>>,
}

impl<P: Platform> Router<P> {
    pub fn builder() -> Builder<P> {
        Builder {
            inner: matchit::Router::new(),
        }
    }

    pub fn at(&self, path: &str) -> Dyn<P> {
        self.inner
            .at(path)
            .map(|a| (a.value)())
            .unwrap_or_else(|_| (self.fallback_fn)())
    }
}

impl<P: Platform> Builder<P> {
    pub fn route<V: View<P> + 'static>(
        mut self,
        route: impl Into<String>,
        view_fn: impl (Fn() -> V) + 'static,
    ) -> Self {
        self.inner
            .insert(route, Box::new(move || Dyn::new(view_fn())))
            .unwrap();
        self
    }

    pub fn or_else<V: View<P> + 'static>(
        self,
        fallback_fn: impl (Fn() -> V) + 'static,
    ) -> Router<P> {
        Router {
            inner: self.inner,
            fallback_fn: Box::new(move || Dyn::new(fallback_fn())),
        }
    }
}
