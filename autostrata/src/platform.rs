use std::{any::Any, fmt::Debug};

use crate::On;

pub trait Platform {
    type Cursor: Any + Clone + Debug;

    fn mark_empty(cursor: &mut Self::Cursor);

    fn new_text(text: &str, cursor: &mut Self::Cursor) -> Handle;
    fn update_text(handle: &mut Handle, text: &str);

    fn new_element(name: &str, cursor: &mut Self::Cursor) -> Handle;

    fn register_event(cursor: &mut Self::Cursor, event: On) -> Handle;

    fn enter_child(cursor: &mut Self::Cursor);
    fn exit_child(cursor: &mut Self::Cursor);

    fn enter_attrs(cursor: &mut Self::Cursor);
    fn exit_attrs(cursor: &mut Self::Cursor);

    fn replace_at_cursor(cursor: &mut Self::Cursor, func: impl FnOnce(&mut Self::Cursor));

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static);

    fn debug_start_reactive_update(cursor: &mut Self::Cursor);
}

pub enum Handle {
    Index(usize),
    Dyn(Box<dyn std::any::Any>),
    #[cfg(feature = "dom")]
    DomNode(web_sys::Node),
    #[cfg(feature = "dom")]
    DomAttr(&'static str),
    #[cfg(feature = "dom")]
    DomEvent(gloo::events::EventListener),
}
