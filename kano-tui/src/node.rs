use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use ratatui::{widgets::Paragraph, Frame};

use crate::{component::ComponentData, tui_state::TuiState};

thread_local! {
    pub(crate) static NEXT_NODE_ID: RefCell<u64> = RefCell::new(0);
}

pub(crate) fn new_node_id() -> u64 {
    NEXT_NODE_ID.with_borrow_mut(|next| {
        let id = *next;
        *next += 1;
        id
    })
}

#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub kind: NodeKind,
    pub on_events: Vec<kano::On>,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub first_child: Option<NodeRef>,
    pub next_sibling: Option<NodeRef>,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Text(String),
    Component(Rc<ComponentData>),
}

#[derive(Clone, Debug)]
pub struct NodeRef(pub Rc<RefCell<Node>>);

impl NodeRef {
    pub fn render(self, tui_state: &mut TuiState, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let node = self.0.borrow();
        match &node.kind {
            NodeKind::Empty => {}
            NodeKind::Text(text) => {
                frame.render_widget(Paragraph::new(text.as_str()), area);
            }
            NodeKind::Component(data) => {
                data.render(self.clone(), tui_state, frame, area);
            }
        }
    }

    pub fn id(&self) -> u64 {
        self.0.borrow().id
    }

    pub fn parent(&self) -> Option<NodeRef> {
        let parent = self.0.borrow().parent.clone();
        parent
            .map(|parent| parent.upgrade().expect("Parent garbage collected"))
            .map(|rc| Self(rc))
    }

    pub fn first_child(&self) -> Option<NodeRef> {
        self.0.borrow().first_child.clone()
    }

    pub fn next_sibling(&self) -> Option<NodeRef> {
        self.0.borrow().next_sibling.clone()
    }

    pub fn append_sibling(&self, kind: NodeKind) {
        let mut self_borrow = self.0.borrow_mut();

        let new_node = Rc::new(RefCell::new(Node {
            id: new_node_id(),
            kind,
            on_events: vec![],
            parent: self_borrow.parent.clone(),
            first_child: None,
            next_sibling: self_borrow.next_sibling.clone(),
        }));

        self_borrow.next_sibling = Some(NodeRef(new_node));
    }
}
