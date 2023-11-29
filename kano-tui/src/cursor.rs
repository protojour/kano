use std::{cell::RefCell, rc::Rc};

use crate::{
    component::ComponentData,
    node::{new_node_id, Node, NodeKind, NodeRef},
};

#[derive(Clone, Debug)]
pub struct TuiCursor {
    location: Location,
    mode: Mode,
}

#[derive(Clone, Debug)]
enum Location {
    Node(NodeRef),
    EndOfChildren(NodeRef),
    Attrs(NodeRef),
}

#[derive(Clone, Debug)]
enum Mode {
    Append,
    Diff,
}

impl TuiCursor {
    pub fn new_root() -> (Self, NodeRef) {
        let root = NodeRef(Rc::new(RefCell::new(Node {
            id: new_node_id(),
            kind: NodeKind::Empty,
            on_events: vec![],
            parent: None,
            first_child: None,
            next_sibling: None,
        })));
        (
            Self {
                location: Location::EndOfChildren(root.clone()),
                mode: Mode::Append,
            },
            root,
        )
    }

    pub fn set_component(&mut self, component: Rc<ComponentData>) {
        self.set_node(NodeKind::Component(component));
    }

    fn set_node(&mut self, kind: NodeKind) {
        match (&self.mode, &self.location) {
            (Mode::Append, Location::Node(node)) => {
                node.append_sibling(kind);
                self.location = Location::Node(node.next_sibling().unwrap());
            }
            (Mode::Append, Location::EndOfChildren(parent)) => {
                let node = Rc::new(RefCell::new(Node {
                    id: new_node_id(),
                    kind,
                    on_events: vec![],
                    parent: Some(Rc::downgrade(&parent.0)),
                    first_child: None,
                    next_sibling: None,
                }));

                if let Some(mut child) = parent.first_child() {
                    // This is a little inefficient
                    while let Some(next) = child.next_sibling() {
                        child = next;
                    }

                    child.0.borrow_mut().next_sibling = Some(NodeRef(node.clone()));
                } else {
                    parent.0.borrow_mut().first_child = Some(NodeRef(node.clone()));
                }

                self.location = Location::Node(NodeRef(node));
            }
            (Mode::Diff, Location::Node(_)) => {}
            (_, Location::Attrs(_)) => {}
            other => todo!("{other:?}"),
        }
    }

    fn current_node(&self) -> NodeRef {
        match &self.location {
            Location::Node(node) => node.clone(),
            _ => panic!(),
        }
    }

    pub fn enter_attrs(&mut self) {
        match &mut self.location {
            Location::Node(node) => {
                self.location = Location::Attrs(node.clone());
            }
            other => panic!("{other:?}"),
        }
    }

    pub fn exit_attrs(&mut self) {
        match &mut self.location {
            Location::Attrs(node) => {
                self.location = Location::Node(node.clone());
            }
            other => panic!("{other:?}"),
        }
    }
}

pub struct TuiEventHandle {
    node: NodeRef,
    event: kano::Event,
}

impl Drop for TuiEventHandle {
    fn drop(&mut self) {
        let mut node_mut = self.node.0.borrow_mut();
        node_mut
            .on_events
            .retain(|on_event| on_event.event() != &self.event);
    }
}

impl kano::platform::Cursor for TuiCursor {
    type TextHandle = NodeRef;
    type EventHandle = TuiEventHandle;

    fn from_text_handle(handle: &NodeRef) -> Self {
        Self {
            location: Location::Node(handle.clone()),
            mode: Mode::Append,
        }
    }

    fn empty(&mut self) {
        self.set_node(NodeKind::Empty);
    }

    fn text(&mut self, text: &str) -> Self::TextHandle {
        self.set_node(NodeKind::Text(text.into()));
        self.current_node()
    }

    fn update_text(&mut self, new_text: &str) {
        match &mut self.location {
            Location::Node(node) => {
                let mut borrow = node.0.borrow_mut();
                if let NodeKind::Text(text) = &mut borrow.kind {
                    *text = new_text.into();
                }
            }
            _ => panic!(),
        }
    }

    fn on_event(&mut self, on_event: kano::On) -> TuiEventHandle {
        match &mut self.location {
            Location::Attrs(node) => {
                let event = *on_event.event();
                {
                    let mut node_mut = node.0.borrow_mut();
                    node_mut.on_events.push(on_event);
                }

                TuiEventHandle {
                    node: node.clone(),
                    event,
                }
            }
            other => panic!("{other:?}"),
        }
    }

    fn enter_children(&mut self) {
        match &self.location {
            Location::Node(node) => {
                self.location = match node.first_child() {
                    Some(first_child) => Location::Node(first_child),
                    None => Location::EndOfChildren(node.clone()),
                }
            }
            Location::Attrs(_) => {}
            other => panic!("{other:?}"),
        }
    }

    fn exit_children(&mut self) {
        match &self.location {
            Location::Node(node) => {
                self.location = Location::Node(node.parent().unwrap());
            }
            Location::EndOfChildren(parent) => {
                self.location = Location::Node(parent.clone());
            }
            Location::Attrs(_) => {}
        }
    }

    fn next_sibling(&mut self) {
        match &self.location {
            Location::Node(node) => {
                self.location = match node.next_sibling() {
                    Some(next) => Location::Node(next),
                    None => match node.parent() {
                        Some(parent) => Location::EndOfChildren(parent),
                        None => Location::Node(node.clone()),
                    },
                }
            }
            Location::EndOfChildren(_) => {}
            Location::Attrs(_) => {}
        }
    }

    fn remove(&mut self) {
        match &self.location {
            Location::Node(node) => {
                let id = node.id();

                let mut prev_sibling: Option<NodeRef> = None;

                if let Some(mut child) = node.parent().and_then(|parent| parent.first_child()) {
                    loop {
                        if child.id() == id {
                            if let Some(prev_sibling) = prev_sibling {
                                prev_sibling.0.borrow_mut().next_sibling = child.next_sibling();
                            } else {
                                node.parent().unwrap().0.borrow_mut().first_child =
                                    child.next_sibling();
                            }
                            return;
                        } else if let Some(next_sibling) = child.next_sibling() {
                            prev_sibling = Some(child);
                            child = next_sibling;
                        } else {
                            return;
                        }
                    }
                }
            }
            Location::EndOfChildren(_) => {}
            _ => panic!(),
        }
    }

    fn enter_diff(&mut self) {
        self.mode = Mode::Diff;
    }

    fn exit_diff(&mut self) {
        self.mode = Mode::Append;
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let (mut replacement_cursor, root_ref) = Self::new_root();
        func(&mut replacement_cursor);

        let node = root_ref.first_child().unwrap();
        let kind = node.0.borrow().kind.clone();

        match &self.location {
            Location::Node(node) => {
                node.0.borrow_mut().kind = kind;
            }
            _ => panic!(),
        }
    }
}
