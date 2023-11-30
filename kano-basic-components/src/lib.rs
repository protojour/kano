use kano::Click;

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "web")]
pub mod web;

/// Props for Kano Basic Components
#[derive(kano::Attribute)]
pub enum KBCProperty {
    OnClick(kano::On<Click>),
}
