use autostrata::{AttrSet, Children, View};
use autostrata_tui::{
    component::{Component, ComponentKind},
    Tui,
};

pub fn layout(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        kind: ComponentKind::Layout,
        children,
    }
}

pub fn paragraph(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        kind: ComponentKind::Paragraph,
        children,
    }
}

pub fn strong(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        kind: ComponentKind::Paragraph,
        children,
    }
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
