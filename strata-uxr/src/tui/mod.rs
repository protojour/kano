use autostrata::{AttrSet, Children, View};
use autostrata_tui::Tui;

pub fn layout(_attrs: (), _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}

pub fn paragraph(_attrs: (), _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}

pub fn strong(_attrs: (), _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}

pub fn button(_attrs: impl AttrSet<Tui>, _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}

pub fn unordered_list(_attrs: (), _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}

pub fn list_item(_attrs: (), _children: impl Children<Tui>) -> impl View<Tui> {
    ()
}
