use std::{fmt::Debug, rc::Rc};

use crate::component::ComponentData;

#[derive(Clone, Debug)]
pub struct NodeData {
    pub kind: NodeKind,
    pub on_events: Vec<kano::OnEvent>,
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            kind: NodeKind::Empty,
            on_events: vec![],
        }
    }
}

impl From<NodeKind> for NodeData {
    fn from(value: NodeKind) -> Self {
        Self {
            kind: value,
            on_events: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Text(String),
    Component(Rc<ComponentData>),
}
