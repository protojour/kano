use std::fmt::Debug;

use crate::{On, View};

pub trait Platform: Sized + 'static {
    type Cursor: Cursor;

    /// Run an application on the platform.
    /// This function _may_ block indefinitely, depending on the platform.
    fn run_app<V: View<Self>, F: (FnOnce() -> V) + 'static>(func: F);

    fn log(s: &str);

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static);
}

pub trait Cursor: Clone + Debug {
    fn from_element_handle(handle: &ElementHandle) -> Self;

    fn empty(&mut self);

    fn text(&mut self, text: &str) -> ElementHandle;
    fn update_text(&mut self, text: &str);

    fn element(&mut self, name: &str) -> ElementHandle;

    fn on_event(&mut self, event: On) -> AttrHandle;

    fn enter_children(&mut self);
    fn exit_children(&mut self);

    fn enter_attrs(&mut self);
    fn exit_attrs(&mut self);

    fn replace(&mut self, func: impl FnOnce(&mut Self));
}

pub enum ElementHandle {
    #[cfg(feature = "web")]
    DomNode(web_sys::Node),
}

pub enum AttrHandle {
    #[cfg(feature = "web")]
    DomAttr(&'static str),
    #[cfg(feature = "web")]
    DomEvent(gloo::events::EventListener),
}
