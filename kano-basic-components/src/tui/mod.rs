use std::rc::Rc;

use kano::prelude::platform::*;
use kano_tui::{
    component::{Component, ComponentData, Layout, StateKeyed, Style},
    ratatui::style::{Color, Modifier},
    Tui,
};

// These Rc's are always constant and could be saved in a thread local.

pub fn layout(attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Block,
            style: Default::default(),
        }),
        attrs,
        children,
    }
}

pub fn paragraph(attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Paragraph,
            style: Default::default(),
        }),
        attrs,
        children,
    }
}

pub fn strong(attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Inline,
            style: Style {
                modifier: Some(StateKeyed::uniform(Modifier::BOLD | Modifier::ITALIC)),
                ..Default::default()
            },
        }),
        attrs,
        children,
    }
}

pub fn button(attrs: impl AttrSet<Tui>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Inline,
            style: Style {
                modifier: Some(StateKeyed::uniform(Modifier::BOLD)),
                fg: Some(StateKeyed {
                    normal: Color::Green,
                    focused: Color::Black,
                }),
                bg: Some(StateKeyed {
                    normal: Color::Black,
                    focused: Color::LightYellow,
                }),
                prefix: Some((
                    "[",
                    Box::new(Style {
                        fg: Some(StateKeyed {
                            normal: Color::Magenta,
                            focused: Color::Black,
                        }),
                        bg: Some(StateKeyed {
                            normal: Color::Black,
                            focused: Color::LightYellow,
                        }),
                        ..Default::default()
                    }),
                )),
                postfix: Some((
                    "]",
                    Box::new(Style {
                        fg: Some(StateKeyed {
                            normal: Color::Magenta,
                            focused: Color::Black,
                        }),
                        bg: Some(StateKeyed {
                            normal: Color::Black,
                            focused: Color::LightYellow,
                        }),
                        ..Default::default()
                    }),
                )),
            },
        }),
        attrs,
        children,
    }
}

pub fn unordered_list(attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Block,
            style: Style::default(),
        }),
        attrs,
        children,
    }
}

pub fn list_item(attrs: (), children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: Rc::new(ComponentData {
            layout: Layout::Paragraph,
            style: Style {
                prefix: Some((
                    "* ",
                    Box::new(Style {
                        fg: Some(StateKeyed::uniform(Color::Red)),
                        ..Default::default()
                    }),
                )),
                ..Style::default()
            },
        }),
        attrs,
        children,
    }
}
