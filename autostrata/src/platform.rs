use std::{any::Any, fmt::Debug};

use crate::On;

pub trait Platform: 'static {
    type Cursor: Cursor;

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static);

    fn debug_start_reactive_update(cursor: &mut Self::Cursor);
}

pub trait Cursor: Any + Clone + Debug {
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
