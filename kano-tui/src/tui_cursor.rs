use std::rc::Rc;

use kano::vdom::{
    vcursor::{Location, Mode, VCursor},
    vnode::VNodeRef,
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

    pub fn enter_attrs(&mut self) {
        self.vcursor.enter_attrs().unwrap();
    }

    pub fn exit_attrs(&mut self) {
        self.vcursor.exit_attrs().unwrap();
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
            .data
            .on_events
            .retain(|on_event| on_event.event() != &self.event);
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

    fn on_event(&mut self, on_event: kano::OnEvent) -> TuiEventHandle {
        match &mut self.vcursor.location {
            Location::Attrs(node) => {
                let event = *on_event.event();
                {
                    let mut node_mut = node.0.borrow_mut();
                    node_mut.data.on_events.push(on_event);
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

    fn enter_diff(&mut self) {
        self.vcursor.enter_diff();
    }

    fn exit_diff(&mut self) {
        self.vcursor.exit_diff();
    }

    fn replace(&mut self, func: impl FnOnce(&mut Self)) {
        let (mut replacement_cursor, root_ref) = Self::new_root();
        func(&mut replacement_cursor);

        self.vcursor.replace(root_ref.first_child().unwrap());
    }
}
