use std::{fmt::Debug, rc::Rc};

use crate::{history::HistoryAPI, View};

pub trait Platform: Sized + 'static {
    type Cursor: Cursor;

    fn init(init: PlatformInit) -> PlatformContext;

    /// This function _may_ block indefinitely, depending on the platform.
    fn run(view: impl View<Self>, context: PlatformContext) -> anyhow::Result<()>;

    fn spawn_task(task: impl std::future::Future<Output = ()> + 'static);
}

pub struct PlatformInit {
    pub signal_dispatch: Box<dyn Fn()>,
    pub history_refresh: Rc<dyn Fn()>,
}

pub struct PlatformContext {
    /// A function that triggers synchronous signal dispatch propagation into the reactive Kano views.
    pub signal_dispatch: Box<dyn Fn()>,
    /// A callback function that Kano calls when there are pending signals to dispatch.
    /// After this function is called, Kano expects an asynchronous call to its signal dispatcher.
    pub on_signal_tick: Rc<dyn Fn()>,
    /// A platform specific logging function.
    pub logger: Rc<dyn Fn(&str)>,
    pub history_api: Rc<dyn HistoryAPI>,
}

/// A cursor used to traverse the UI tree on a given platform.
pub trait Cursor: Clone + Debug {
    type TextHandle: 'static;
    type EventHandle: 'static;

    fn from_text_handle(handle: &Self::TextHandle) -> Self;

    fn empty(&mut self);

    fn text(&mut self, text: &str) -> Self::TextHandle;
    fn update_text(&mut self, text: &str);

    fn enter_children(&mut self);
    fn exit_children(&mut self);
    fn next_sibling(&mut self);
    fn remove(&mut self);

    fn replace(&mut self, func: impl FnOnce(&mut Self));
}

#[cfg(test)]
pub(crate) mod test_platform {
    use std::rc::Rc;

    use crate::{history::HistoryState, View};

    use super::{Cursor, PlatformContext, PlatformInit};

    pub struct TestPlatform;

    impl super::Platform for TestPlatform {
        type Cursor = ();

        fn init(init: PlatformInit) -> PlatformContext {
            PlatformContext {
                on_signal_tick: Rc::new(|| {}),
                signal_dispatch: init.signal_dispatch,
                logger: Rc::new(|_| {}),
                history_api: Rc::new(HistoryState::new("".to_string())),
            }
        }

        fn run(_view: impl View<Self>, _context: PlatformContext) -> anyhow::Result<()> {
            Ok(())
        }

        fn spawn_task(_task: impl std::future::Future<Output = ()> + 'static) {}
    }

    impl Cursor for () {
        type TextHandle = ();
        type EventHandle = ();

        fn from_text_handle(_handle: &Self::TextHandle) -> Self {}
        fn empty(&mut self) {}
        fn text(&mut self, _text: &str) -> Self::TextHandle {}
        fn update_text(&mut self, _text: &str) {}
        fn enter_children(&mut self) {}
        fn exit_children(&mut self) {}
        fn next_sibling(&mut self) {}
        fn remove(&mut self) {}
        fn replace(&mut self, _func: impl FnOnce(&mut Self)) {}
    }
}
