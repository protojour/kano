use std::rc::Rc;

use kano::{AttrSet, Children, View};
use kano_tui::{
    component::{Component, ComponentData, Layout, Style},
    ratatui::style::{Color, Modifier},
    Tui,
};

// These Rc's are always constant and could be saved in a thread local.

pub fn layout(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Block,
            style: Default::default(),
        }),
        children,
    }
}

pub fn paragraph(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Paragraph,
            style: Default::default(),
        }),
        children,
    }
}

pub fn strong(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Inline,
            style: Style {
                modifier: Some(Modifier::BOLD | Modifier::ITALIC),
                ..Default::default()
            },
        }),
        children,
    }
}

pub fn button(_attrs: impl AttrSet<Tui>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Inline,
            style: Style {
                modifier: Some(Modifier::BOLD),
                fg: Some(Color::Green),
                prefix: Some((
                    "[",
                    Box::new(Style {
                        fg: Some(Color::Magenta),
                        ..Default::default()
                    }),
                )),
                postfix: Some((
                    "]",
                    Box::new(Style {
                        fg: Some(Color::Magenta),
                        ..Default::default()
                    }),
                )),
                ..Default::default()
            },
        }),
        children,
    }
}

pub fn unordered_list(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Block,
            style: Style::default(),
        }),
        children,
    }
}

pub fn list_item(_attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Paragraph,
            style: Style {
                prefix: Some((
                    "* ",
                    Box::new(Style {
                        fg: Some(Color::Red),
                        ..Default::default()
                    }),
                )),
                ..Style::default()
            },
        }),
        children,
    }
}
