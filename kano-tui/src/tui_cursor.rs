use std::rc::Rc;

use kano::{
    vdom::{
        vcursor::{Location, Mode, VCursor},
        vnode::VNodeRef,
    },
    Click, On,
};

use crate::{
    component::ComponentData,
    node_data::{NodeData, NodeKind},
};

type NodeRef = VNodeRef<NodeData>;

#[derive(Clone, Debug)]
pub struct TuiCursor {
    vcursor: VCursor<NodeData>,
}

impl TuiCursor {
    pub fn new_root() -> (Self, VNodeRef<NodeData>) {
        let (v_cursor, node_ref) = VCursor::new_root();
        (Self { vcursor: v_cursor }, node_ref)
    }

    pub fn set_component(&mut self, component: Rc<ComponentData>) {
        self.vcursor
            .put_node(NodeData::from(NodeKind::Component(component)));
    }

    fn current_node(&self) -> VNodeRef<NodeData> {
        match &self.vcursor.location {
            Location::Node(node) => node.clone(),
            _ => panic!(),
        }
    }

    pub fn set_on_click(&mut self, on_click: Option<On<Click>>) {
        match &mut self.vcursor.location {
            Location::Node(node) => {
                let mut node_mut = node.0.borrow_mut();
                node_mut.data.on_click = on_click;
            }
            other => panic!("{other:?}"),
        }
    }
}

pub struct TuiEventHandle {
    node: NodeRef,
}

impl Drop for TuiEventHandle {
    fn drop(&mut self) {
        let mut node_mut = self.node.0.borrow_mut();
        node_mut.data.on_click = None;
    }
}

impl kano::platform::Cursor for TuiCursor {
    type TextHandle = NodeRef;
    type EventHandle = TuiEventHandle;

    fn from_text_handle(handle: &VNodeRef<NodeData>) -> Self {
        Self {
            vcursor: VCursor {
                location: Location::Node(handle.clone()),
                mode: Mode::Append,
            },
        }
    }

    fn empty(&mut self) {
        self.vcursor.put_node(NodeData::from(NodeKind::Empty));
    }

    fn text(&mut self, text: &str) -> Self::TextHandle {
        self.vcursor
            .put_node(NodeData::from(NodeKind::Text(text.into())));
        self.current_node()
    }

    fn update_text(&mut self, new_text: &str) {
        match &mut self.vcursor.location {
            Location::Node(node) => {
                let mut borrow = node.0.borrow_mut();
                if let NodeKind::Text(text) = &mut borrow.data.kind {
                    *text = new_text.into();
                }
            }
            _ => panic!(),
        }
    }

    fn enter_children(&mut self) {
        self.vcursor.enter_children().unwrap();
    }

    fn exit_children(&mut self) {
        self.vcursor.exit_children().unwrap();
    }

    fn next_sibling(&mut self) {
        self.vcursor.next_sibling();
    }

    fn remove(&mut self) {
        self.vcursor.remove();
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let (mut replacement_cursor, root_ref) = Self::new_root();
        func(&mut replacement_cursor);

        self.vcursor.replace(root_ref.first_child().unwrap());
    }
}
