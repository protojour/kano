use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

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
pub struct VNode<T> {
    pub id: u64,
    pub data: T,
    pub parent: Option<Weak<RefCell<VNode<T>>>>,
    pub first_child: Option<VNodeRef<T>>,
    pub next_sibling: Option<VNodeRef<T>>,
}

#[derive(Debug)]
pub struct VNodeRef<T>(pub Rc<RefCell<VNode<T>>>);

impl<T> Clone for VNodeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> VNodeRef<T> {
    pub fn id(&self) -> u64 {
        self.0.borrow().id
    }

    pub fn parent(&self) -> Option<VNodeRef<T>> {
        let parent = self.0.borrow().parent.clone();
        parent
            .map(|parent| parent.upgrade().expect("Parent garbage collected"))
            .map(Self)
    }

    pub fn first_child(&self) -> Option<VNodeRef<T>> {
        self.0.borrow().first_child.clone()
    }

    pub fn next_sibling(&self) -> Option<VNodeRef<T>> {
        self.0.borrow().next_sibling.clone()
    }

    pub fn append_sibling(&self, data: T) {
        let mut self_borrow = self.0.borrow_mut();
        assert!(self_borrow.next_sibling.is_none());

        let new_node = Rc::new(RefCell::new(VNode {
            id: new_node_id(),
            data,
            parent: self_borrow.parent.clone(),
            first_child: None,
            next_sibling: self_borrow.next_sibling.clone(),
        }));

        self_borrow.next_sibling = Some(VNodeRef(new_node));
    }

    pub fn has_child(&self, child_id: u64) -> bool {
        let mut next_child = self.first_child();

        while let Some(child) = next_child {
            if child.id() == child_id {
                return true;
            }

            next_child = child.next_sibling();
        }

        false
    }
}
