use kano::Click;

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "web")]
pub mod web;

/// Props for Kano Basic Components
#[derive(kano::FromProperty)]
pub enum KBCAttributes {
    OnClick(kano::On<Click>),
}
