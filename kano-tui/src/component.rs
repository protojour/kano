use std::rc::Rc;

use kano::{
    vdom::vnode::{VNode, VNodeRef},
    Children, Click, On,
};
use ratatui::{
    style::{Color, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::{
    node_data::{NodeData, NodeKind},
    tui_cursor::TuiCursor,
    tui_state::TuiState,
    Tui,
};

#[derive(Clone)]
pub struct Component<C> {
    pub data: Rc<ComponentData>,
    pub on_click: Option<On<Click>>,
    pub children: C,
}

impl<C: Children<Tui>> kano::Diff<Tui> for Component<C> {
    type State = (Rc<ComponentData>, C::State);

    fn init(self, cursor: &mut TuiCursor) -> Self::State {
        cursor.set_component(self.data.clone());

        cursor.set_on_click(self.on_click.clone());

        let children_state = self.children.init(cursor);

        (self.data, children_state)
    }

    fn diff(self, state: &mut Self::State, cursor: &mut TuiCursor) {
        cursor.set_on_click(self.on_click);
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

#[derive(Clone, Copy)]
pub enum StyleState {
    Normal,
    Focused,
}

#[derive(Clone, Default, Debug)]
pub struct Style {
    pub modifier: Option<StateKeyed<Modifier>>,
    pub fg: Option<StateKeyed<Color>>,
    pub bg: Option<StateKeyed<Color>>,
    pub prefix: Option<(&'static str, Box<Style>)>,
    pub postfix: Option<(&'static str, Box<Style>)>,
}

#[derive(Clone, Debug)]
pub struct StateKeyed<T> {
    pub normal: T,
    pub focused: T,
}

impl<T> StateKeyed<T> {
    pub fn uniform(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            normal: value.clone(),
            focused: value,
        }
    }

    pub fn for_state(&self, state: StyleState) -> &T {
        match state {
            StyleState::Normal => &self.normal,
            StyleState::Focused => &self.focused,
        }
    }
}

impl ComponentData {
    pub fn render(
        &self,
        node: VNodeRef<NodeData>,
        tui_state: &mut TuiState,
        frame: &mut Frame,
        area: ratatui::prelude::Rect,
    ) {
        let mut collector = Collector {
            spans: vec![],
            lines: vec![],
            tui_state,
            style_state: StyleState::Normal,
        };
        collector.collect_lines(node, Default::default());

        if !collector.spans.is_empty() {
            collector.lines.push(Line::from(collector.spans));
        }

        frame.render_widget(
            Paragraph::new(Text::from(collector.lines))
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

pub fn all_children(node: VNodeRef<NodeData>) -> Vec<VNodeRef<NodeData>> {
    let mut output = vec![];
    let mut next_child = node.first_child();

    while let Some(child) = next_child {
        output.push(child.clone());
        next_child = child.next_sibling();
    }

    output
}

struct Collector<'t, 's> {
    spans: Vec<Span<'t>>,
    lines: Vec<Line<'t>>,
    tui_state: &'s mut TuiState,
    style_state: StyleState,
}

impl<'t, 's> Collector<'t, 's> {
    fn collect_lines(&mut self, node: VNodeRef<NodeData>, tui_style: ratatui::style::Style) {
        let node_borrow = node.0.borrow();

        match &node_borrow.data.kind {
            NodeKind::Empty => {}
            NodeKind::Text(text) => {
                self.spans.push(Span::styled(text.clone(), tui_style));
            }
            NodeKind::Component(data) => {
                match &data.layout {
                    Layout::Block | Layout::Paragraph => {
                        if !self.spans.is_empty() {
                            self.lines.push(Line::from(std::mem::take(&mut self.spans)));
                        }
                    }
                    Layout::Inline => {}
                }

                let mut unfocus = false;
                if let Some(click_handler) = find_click_handler(&node_borrow) {
                    if self.tui_state.focusable_counter == self.tui_state.currently_focused {
                        self.tui_state.focused_event_handler = Some(click_handler);
                        self.style_state = StyleState::Focused;
                        unfocus = true;
                    }

                    self.tui_state.focusable_counter += 1;
                }

                if let Some((prefix, style)) = &data.style.prefix {
                    let mut prefix_style = tui_style;
                    apply_style(&mut prefix_style, style, self.style_state);
                    self.spans.push(Span::styled(*prefix, prefix_style));
                }

                let mut sub_style = tui_style;
                apply_style(&mut sub_style, &data.style, self.style_state);

                let mut next_child = node.first_child();

                while let Some(child) = next_child {
                    self.collect_lines(child.clone(), sub_style);
                    next_child = child.next_sibling();
                }

                if let Some((postfix, style)) = &data.style.postfix {
                    let mut postfix_style = tui_style;
                    apply_style(&mut postfix_style, style, self.style_state);
                    self.spans.push(Span::styled(*postfix, postfix_style));
                }

                if unfocus {
                    self.style_state = StyleState::Normal;
                }
            }
        }
    }
}

fn apply_style(tui_style: &mut ratatui::style::Style, style: &Style, state: StyleState) {
    if let Some(modifier) = &style.modifier {
        *tui_style = tui_style.add_modifier(*modifier.for_state(state));
    }
    if let Some(fg) = &style.fg {
        *tui_style = tui_style.fg(*fg.for_state(state));
    }
    if let Some(bg) = &style.bg {
        *tui_style = tui_style.bg(*bg.for_state(state));
    }
}

fn find_click_handler(node: &VNode<NodeData>) -> Option<On<Click>> {
    node.data.on_click.clone()
}
