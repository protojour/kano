/// Prelude for apps which excludes platform-agnostic types.
pub mod app {
    pub use crate::let_props;
    pub use crate::platform::Platform;
    pub use crate::reactive::*;
    pub use crate::view::Dyn;
    pub use crate::view::Fmt;
    pub use kano_macros::view;
}

/// Prelude for platforms that extend the app prelude, but extends it with platform-agnostic types.
pub mod platform {
    pub use super::app::*;
    pub use crate::{Children, Diff, Props, View};
}
