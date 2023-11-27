use crate::registry::REGISTRY;

/// A ViewId is assigned to views that do "smart things",
///
/// This includes Reactive views and other views that involves user-defined functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ViewId(pub(crate) u64);

impl ViewId {
    /// Set the reactive id as current in a reactive operation,
    /// and execute the given function, before resetting state to previous state again.
    ///
    /// Setting a reactive to the current one, enables
    /// automatic subscription creation when a signal dependency is registered.
    pub(crate) fn as_current_reactive<T>(self, func: impl FnOnce() -> T) -> T {
        let (prev_reactive, prev_func) = REGISTRY.with_borrow_mut(|registry| {
            // FIXME(?): Not backing up the signal position tracker.
            // The hypothesis is that the view should not continue to create signals
            // after it has drawn its children.
            // FIXME: Make runtime assertion for this invariant
            registry.current_func_view_signal_tracker = 0;

            (
                registry.current_reactive_view.replace(self),
                registry.current_func_view.replace(self),
            )
        });

        let value = func();

        REGISTRY.with_borrow_mut(|registry| {
            registry.current_reactive_view = prev_reactive;
            registry.current_func_view = prev_func;
        });

        value
    }

    pub(crate) fn as_current_func<T>(self, func: impl FnOnce() -> T) -> T {
        let prev_func = REGISTRY.with_borrow_mut(|registry| {
            // FIXME(?): Not backing up the signal position tracker.
            registry.current_func_view_signal_tracker = 0;

            registry.current_func_view.replace(self)
        });

        let value = func();

        REGISTRY.with_borrow_mut(|registry| {
            registry.current_func_view = prev_func;
        });

        value
    }
}
