use std::any::Any;

use crate::On;

pub trait Platform {
    type Cursor: Clone + Any;

    fn new_text(text: &str, cursor: &mut Self::Cursor) -> Handle;
    fn update_text(handle: &mut Handle, text: &str);

    fn new_element(cursor: &mut Self::Cursor, name: &str) -> Handle;

    fn register_event(cursor: &mut Self::Cursor, event: On) -> Handle;

    fn enter_child(cursor: &mut Self::Cursor);
    fn exit_child(cursor: &mut Self::Cursor);

    fn enter_attrs(cursor: &mut Self::Cursor);
    fn exit_attrs(cursor: &mut Self::Cursor);

    fn unmount(handle: &mut Handle, cursor: &mut Self::Cursor);

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static);
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
