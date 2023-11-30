#[cfg(feature = "tui")]
pub mod tui;

#[cfg(feature = "web")]
pub mod web;

/// Props for Kano Basic Components
pub enum KBCProperty {
    OnEvent(kano::OnEvent),
}

impl kano::Attribute<KBCProperty> for kano::OnEvent {
    fn into_prop(self) -> Option<KBCProperty> {
        Some(KBCProperty::OnEvent(self))
    }
}
