use crate::{
    markup::Markup,
    prelude::platform::Platform,
    view::{Dyn, Reactive},
    View,
};

pub type Router<P> = StackRouter<P, <P as Platform>::Markup>;

pub struct StackRouter<P: Platform, M: Markup<P>> {
    inner: matchit::Router<Box<dyn Fn() -> Dyn<P, M>>>,
    fallback_fn: Box<dyn Fn() -> Dyn<P, M>>,
}

pub struct Builder<P: Platform, M: Markup<P>> {
    inner: matchit::Router<Box<dyn Fn() -> Dyn<P, M>>>,
}

impl<P: Platform, M: Markup<P>> StackRouter<P, M> {
    pub fn builder() -> Builder<P, M> {
        Builder {
            inner: matchit::Router::new(),
        }
    }

    pub fn at(&self, path: &str) -> Dyn<P, M> {
        crate::log(&format!("Router::at(\"{path}\")"));
        self.inner
            .at(path)
            .map(|match_| (match_.value)())
            .unwrap_or_else(|_| (self.fallback_fn)())
    }
}

impl<P: Platform, M: Markup<P>> Builder<P, M> {
    pub fn route<V: View<P, M> + 'static>(
        mut self,
        route: impl Into<String>,
        view_fn: impl (Fn() -> V) + Copy + 'static,
    ) -> Self {
        self.inner
            .insert(route, Box::new(move || Dyn::new(Reactive(view_fn))))
            .unwrap();
        self
    }

    pub fn or_else<V: View<P, M> + 'static>(
        self,
        fallback_fn: impl (Fn() -> V) + Copy + 'static,
    ) -> StackRouter<P, M> {
        StackRouter {
            inner: self.inner,
            fallback_fn: Box::new(move || Dyn::new(Reactive(fallback_fn))),
        }
    }
}
