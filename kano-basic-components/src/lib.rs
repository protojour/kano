use kano::attr::{Click, On, To};

#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "web")]
pub mod web;

/// Props for Kano Basic Components
#[derive(kano::FromProperty)]
pub enum KBCAttr {
    OnClick(On<Click>),
    To(To),
}
