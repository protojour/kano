use std::rc::Rc;

use kano::Children;
use ratatui::{
    style::{Color, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::{
    node::{NodeKind, NodeRef},
    Tui,
};

#[derive(Clone)]
pub struct Component<C> {
    pub data: Rc<ComponentData>,
    pub children: C,
}

impl<C: Children<Tui>> kano::Diff<Tui> for Component<C> {
    type State = (Rc<ComponentData>, C::State);

    fn init(self, cursor: &mut <Tui as kano::prelude::Platform>::Cursor) -> Self::State {
        cursor.set_component(self.data.clone());
        let children_state = self.children.init(cursor);

        (self.data, children_state)
    }

    fn diff(self, state: &mut Self::State, cursor: &mut <Tui as kano::prelude::Platform>::Cursor) {
        self.children.diff(&mut state.1, cursor);
    }
}

impl<C: Children<Tui>> kano::View<Tui> for Component<C> {}

#[derive(Clone, Debug)]
pub struct ComponentData {
    pub layout: Layout,
    pub style: Style,
}

#[derive(Clone, Debug)]
pub enum Layout {
    Block,
    Paragraph,
    Inline,
}

#[derive(Clone, Default, Debug)]
pub struct Style {
    pub modifier: Option<Modifier>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub prefix: Option<(&'static str, Box<Style>)>,
    pub postfix: Option<(&'static str, Box<Style>)>,
}

impl ComponentData {
    pub fn render(&self, node: NodeRef, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let mut spans = vec![];
        let mut lines = vec![];

        collect_lines(node, &mut spans, &mut lines, Default::default());

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }

        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Kano TUI. Press q to quit.")
                        .borders(Borders::ALL)
                        .border_set(ratatui::symbols::border::DOUBLE)
                        .padding(Padding::uniform(1)),
                ),
            area,
        );
    }
}

pub fn all_children(node: NodeRef) -> Vec<NodeRef> {
    let mut output = vec![];
    let mut next_child = node.first_child();

    while let Some(child) = next_child {
        output.push(child.clone());
        next_child = child.next_sibling();
    }

    output
}

pub fn text_children(node: NodeRef) -> String {
    let mut buf = String::new();

    let mut next_child = node.first_child();

    while let Some(child) = next_child {
        match &child.0.borrow().kind {
            NodeKind::Text(text) => {
                buf.push_str(&text);
            }
            _ => {}
        }

        next_child = child.next_sibling();
    }

    buf
}

fn collect_lines<'a>(
    node: NodeRef,
    spans: &mut Vec<Span<'a>>,
    lines: &mut Vec<Line<'a>>,
    tui_style: ratatui::style::Style,
) {
    match &node.0.borrow().kind {
        NodeKind::Empty => {}
        NodeKind::Text(text) => {
            spans.push(Span::styled(text.clone(), tui_style));
        }
        NodeKind::Component(data) => {
            match &data.layout {
                Layout::Block | Layout::Paragraph => {
                    if !spans.is_empty() {
                        lines.push(Line::from(std::mem::take(spans)));
                    }
                }
                Layout::Inline => {}
            }

            if let Some((prefix, style)) = &data.style.prefix {
                let mut tui_style = tui_style.clone();
                apply_style(&mut tui_style, &style);
                spans.push(Span::styled(*prefix, tui_style));
            }

            let mut sub_style = tui_style.clone();
            apply_style(&mut sub_style, &data.style);

            let mut next_child = node.first_child();

            while let Some(child) = next_child {
                collect_lines(child.clone(), spans, lines, sub_style);
                next_child = child.next_sibling();
            }

            if let Some((postfix, style)) = &data.style.postfix {
                let mut tui_style = tui_style.clone();
                apply_style(&mut tui_style, &style);
                spans.push(Span::styled(*postfix, tui_style));
            }
        }
    }
}

fn apply_style(tui_style: &mut ratatui::style::Style, style: &Style) {
    if let Some(modifier) = &style.modifier {
        *tui_style = tui_style.add_modifier(*modifier);
    }
    if let Some(fg) = &style.fg {
        *tui_style = tui_style.fg(*fg);
    }
    if let Some(bg) = &style.bg {
        *tui_style = tui_style.bg(*bg);
    }
}
