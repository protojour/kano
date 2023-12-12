use std::fmt::Debug;

/// A UI markup language.
///
/// Kano's abstraction for building Graphical User Interfaces is to build
/// trees using different markup languages.
///
/// Markup is generic over the platform it is implemented on.
pub trait Markup<P>: Sized + 'static {
    type Cursor: Cursor;
}

/// A trait that says that some [Markup] language `M` can be nested inside this markup language,
/// and provides implementation of this for a specific [crate::platform::Platform] `P`.
pub trait NestMarkup<P, M>: Markup<P> {
    type Nested: Markup<P>;

    fn nest(cursor: &mut Self::Cursor) -> <Self::Nested as Markup<P>>::Cursor;
    fn unnest(nested: <Self::Nested as Markup<P>>::Cursor, original: &mut Self::Cursor);
}

/// A cursor used to traverse some markup language on a given [crate::platform::Platform].
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
