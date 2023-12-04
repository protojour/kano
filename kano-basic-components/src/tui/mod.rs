use std::{cell::RefCell, collections::HashMap, rc::Rc};

use kano::{prelude::platform::*, Empty};
use kano_tui::{
    component::{Component, ComponentData, Layout, StateKeyed, Style},
    ratatui::style::{Color, Modifier},
    Tui,
};

use crate::{KBCAttr, To};

pub fn layout(_: impl Props<Empty>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: data("layout", || ComponentData {
            layout: Layout::Block,
            style: Default::default(),
        }),
        on_click: None,
        children,
    }
}

pub fn paragraph(_: impl Props<Empty>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: data("paragraph", || ComponentData {
            layout: Layout::Paragraph,
            style: Default::default(),
        }),
        on_click: None,
        children,
    }
}

pub fn strong(_: impl Props<Empty>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: data("strong", || ComponentData {
            layout: Layout::Inline,
            style: Style {
                modifier: Some(StateKeyed::uniform(Modifier::BOLD | Modifier::ITALIC)),
                ..Default::default()
            },
        }),
        on_click: None,
        children,
    }
}

pub fn button(mut props: impl Props<KBCAttr>, children: impl Children<Tui>) -> impl View<Tui> {
    let_props!({ KBCAttr::OnClick(on_click), KBCAttr::To(to) } = props);

    if let Some(To(location)) = to {
        on_click = Some(on::click(move || {
            kano::history::push(location.clone().into_owned());
        }));
    }

    Component {
        data: data("button", || ComponentData {
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
        on_click,
        children,
    }
}

pub fn unordered_list(_: impl Props<Empty>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: data("unordered_list", || ComponentData {
            layout: Layout::Block,
            style: Style::default(),
        }),
        on_click: None,
        children,
    }
}

pub fn list_item(_: impl Props<Empty>, children: impl Children<Tui>) -> impl View<Tui> {
    Component {
        data: data("list_item", || ComponentData {
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
        on_click: None,
        children,
    }
}

thread_local! {
    static DATA_CACHE: RefCell<HashMap<&'static str, Rc<ComponentData>>> = RefCell::new(HashMap::new());
}

fn data(key: &'static str, producer: impl FnOnce() -> ComponentData) -> Rc<ComponentData> {
    DATA_CACHE.with_borrow_mut(|cache| {
        cache
            .entry(key)
            .or_insert_with(|| Rc::new(producer()))
            .clone()
    })
}
