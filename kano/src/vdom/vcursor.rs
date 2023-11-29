use std::{cell::RefCell, rc::Rc};

use super::vnode::{new_node_id, VNode, VNodeRef};

#[derive(Clone, Debug)]
pub struct VCursor<T> {
    pub location: Location<T>,
    pub mode: Mode,
}

#[derive(Clone, Debug)]
pub enum Location<T> {
    Node(VNodeRef<T>),
    EndOfChildren(VNodeRef<T>),
    Attrs(VNodeRef<T>),
}

#[derive(Clone, Debug)]
pub enum Mode {
    Append,
    Diff,
}

impl<T> VCursor<T> {
    pub fn new_root() -> (Self, VNodeRef<T>)
    where
        T: Default,
    {
        let root = VNodeRef(Rc::new(RefCell::new(VNode {
            id: new_node_id(),
            data: T::default(),
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

    pub fn put_node(&mut self, data: T) {
        match (&self.mode, &self.location) {
            (Mode::Append, Location::Node(node)) => {
                node.append_sibling(data);
                self.location = Location::Node(node.next_sibling().unwrap());
            }
            (Mode::Diff, Location::Node(node)) => {
                node.0.borrow_mut().data = data;
            }
            (Mode::Append, Location::EndOfChildren(parent)) => {
                let node = Rc::new(RefCell::new(VNode {
                    id: new_node_id(),
                    data,
                    parent: Some(Rc::downgrade(&parent.0)),
                    first_child: None,
                    next_sibling: None,
                }));

                if let Some(mut child) = parent.first_child() {
                    // This is a little inefficient
                    while let Some(next) = child.next_sibling() {
                        child = next;
                    }

                    child.0.borrow_mut().next_sibling = Some(VNodeRef(node.clone()));
                } else {
                    parent.0.borrow_mut().first_child = Some(VNodeRef(node.clone()));
                }

                self.location = Location::Node(VNodeRef(node));
            }
            (_, Location::Attrs(_)) => {}
            (Mode::Diff, Location::EndOfChildren(_)) => panic!("Diff at end of children"),
        }
    }

    pub fn enter_children(&mut self) -> Result<(), &mut Self> {
        match &self.location {
            Location::Node(node) => {
                self.location = match node.first_child() {
                    Some(first_child) => Location::Node(first_child),
                    None => Location::EndOfChildren(node.clone()),
                };
                Ok(())
            }
            Location::Attrs(_) => Ok(()),
            _ => Err(self),
        }
    }

    pub fn exit_children(&mut self) -> Result<(), &mut Self> {
        match &self.location {
            Location::Node(node) => {
                self.location = Location::Node(node.parent().unwrap());
                Ok(())
            }
            Location::EndOfChildren(parent) => {
                self.location = Location::Node(parent.clone());
                Ok(())
            }
            Location::Attrs(_) => Ok(()),
        }
    }

    pub fn next_sibling(&mut self) {
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

    pub fn enter_attrs(&mut self) -> Result<(), &mut Self> {
        match &mut self.location {
            Location::Node(node) => {
                self.location = Location::Attrs(node.clone());
                Ok(())
            }
            _ => Err(self),
        }
    }

    pub fn exit_attrs(&mut self) -> Result<(), &mut Self> {
        match &mut self.location {
            Location::Attrs(node) => {
                self.location = Location::Node(node.clone());
                Ok(())
            }
            _ => Err(self),
        }
    }

    pub fn enter_diff(&mut self) {
        self.mode = Mode::Diff;
    }

    pub fn exit_diff(&mut self) {
        self.mode = Mode::Append;
    }

    pub fn remove(&mut self) {
        match &self.location {
            Location::Node(node) => {
                let id = node.id();

                let mut prev_sibling: Option<VNodeRef<T>> = None;

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

    pub fn replace(&mut self, new_node: VNodeRef<T>)
    where
        T: Default,
    {
        let Location::Node(old_node) = &self.location else {
            panic!();
        };

        let old_id = old_node.id();
        let parent = old_node.parent().unwrap();

        let mut prev_sibling: Option<VNodeRef<T>> = None;
        let mut next_sibling = parent.first_child();

        while let Some(child) = next_sibling {
            if child.id() == old_id {
                {
                    let mut new_node_mut = new_node.0.borrow_mut();
                    new_node_mut.next_sibling = child.next_sibling();
                    new_node_mut.parent = Some(Rc::downgrade(&parent.0));
                }

                if let Some(prev_sibling) = prev_sibling {
                    prev_sibling.0.borrow_mut().next_sibling = Some(new_node.clone());
                } else {
                    parent.0.borrow_mut().first_child = Some(new_node.clone());
                }

                self.location = Location::Node(new_node.clone());

                return;
            }

            next_sibling = child.next_sibling();
            prev_sibling = Some(child);
        }

        panic!("Child {old_id} not found");
    }
}
