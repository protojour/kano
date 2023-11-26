use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use ratatui::widgets::Paragraph;

pub struct Node {
    pub kind: NodeKind,
    parent: Weak<RefCell<Node>>,
    first_child: Rc<RefCell<Option<Node>>>,
    next_sibling: Rc<RefCell<Option<Node>>>,
}

pub trait TuiElement {
    fn render(
        &self,
        node: &Node,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
    }
}

pub enum NodeKind {
    Empty,
    Text(String),
    Element(Box<dyn TuiElement>),
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
            NodeKind::Element(tui_element) => {
                tui_element.render(&node, area, buf);
            }
        }
    }
}
