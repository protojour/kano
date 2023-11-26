use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use ratatui::widgets::Paragraph;

use crate::widget::Widget;

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    parent: Weak<RefCell<Node>>,
    first_child: Rc<RefCell<Option<Node>>>,
    next_sibling: Rc<RefCell<Option<Node>>>,
}

#[derive(Debug)]
pub enum NodeKind {
    Empty,
    Text(String),
    Widget(Widget),
}

pub struct NodeRef(Rc<RefCell<Node>>);

impl ratatui::widgets::Widget for NodeRef {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let node = self.0.borrow();
        match &node.kind {
            NodeKind::Empty => {}
            NodeKind::Text(text) => {
                Paragraph::new(text.as_str()).render(area, buf);
            }
            NodeKind::Widget(tui_element) => {
                tui_element.render(&node, area, buf);
            }
        }
    }
}
