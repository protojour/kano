use std::{fmt::Debug, rc::Rc};

use kano::{Click, On};

use crate::component::ComponentData;

#[derive(Clone, Debug)]
pub struct NodeData {
    pub kind: NodeKind,
    pub on_click: Option<On<Click>>,
}

impl Default for NodeData {
    fn default() -> Self {
        Self {
            kind: NodeKind::Empty,
            on_click: None,
        }
    }
}

impl From<NodeKind> for NodeData {
    fn from(value: NodeKind) -> Self {
        Self {
            kind: value,
            on_click: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Text(String),
    Component(Rc<ComponentData>),
}
