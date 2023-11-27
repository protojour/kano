use autostrata::Children;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{
    node::{NodeKind, NodeRef},
    Tui,
};

#[derive(Clone)]
pub struct Component<C> {
    pub kind: ComponentKind,
    pub children: C,
}

impl<C: Children<Tui>> autostrata::Diff<Tui> for Component<C> {
    type State = (ComponentKind, C::State);

    fn init(self, cursor: &mut <Tui as autostrata::prelude::Platform>::Cursor) -> Self::State {
        cursor.set_component(self.kind.clone());
        let children_state = self.children.init(cursor);

        (self.kind, children_state)
    }

    fn diff(
        self,
        state: &mut Self::State,
        cursor: &mut <Tui as autostrata::prelude::Platform>::Cursor,
    ) {
        self.children.diff(&mut state.1, cursor);
    }
}

impl<C: Children<Tui>> autostrata::View<Tui> for Component<C> {}

#[derive(Clone, Debug)]
pub enum ComponentKind {
    Layout,
    Paragraph,
    Button,
}

impl ComponentKind {
    pub fn render(&self, node: NodeRef, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let mut text = String::new();
        all_text_under(node.clone(), &mut text);

        match self {
            Self::Layout => {
                for (index, child) in all_children(node).into_iter().enumerate() {
                    let area = Rect::new(0, index as u16, frame.size().width, 1);

                    child.render(frame, area);
                }
            }
            Self::Paragraph => {
                let text = text_children(node);
                frame.render_widget(Paragraph::new(text), area);
            }
            Self::Button => {}
        }
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

pub fn all_text_under(node: NodeRef, buf: &mut String) {
    let mut next_child = node.first_child();

    while let Some(child) = next_child {
        match &child.0.borrow().kind {
            NodeKind::Empty => {}
            NodeKind::Text(text) => {
                buf.push_str(&text);
            }
            NodeKind::Component(_) => {
                if let Some(first_child) = child.first_child() {
                    all_text_under(first_child, buf);
                }
            }
        }

        next_child = child.next_sibling();
    }
}

pub fn count_nodes(node: NodeRef, out: &mut usize, dbg: &mut String) {
    *out += 1;

    dbg.push_str("N");

    let mut next_child = node.first_child();

    while let Some(child) = next_child {
        dbg.push_str("C");
        count_nodes(child.clone(), out, dbg);
        next_child = child.next_sibling();
    }
}
