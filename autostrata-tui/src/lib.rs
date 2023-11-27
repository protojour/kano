use autostrata::Diff;
use component::ComponentData;
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use node::{new_node_id, Node, NodeKind, NodeRef};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    cell::RefCell,
    io::{self, stdout},
    panic,
    rc::Rc,
};

pub mod component;
pub mod node;

pub use ratatui;

pub struct Tui;

impl autostrata::platform::Platform for Tui {
    type Cursor = TuiCursor;

    fn log(_s: &str) {}

    fn run_app<V: autostrata::View<Self>, F: (FnOnce() -> V) + 'static>(
        func: F,
    ) -> anyhow::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            reset_terminal().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        let (mut cursor, empty_root) = TuiCursor::new_root();
        let state = autostrata::view::Func(func, ()).init(&mut cursor);
        std::mem::forget(state);

        let root_node = empty_root.first_child().unwrap();

        loop {
            terminal.draw(|frame| {
                let area = frame.size();
                root_node.clone().render(frame, area);
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        stdout().execute(LeaveAlternateScreen)?;
        reset_terminal()?;
        Ok(())
    }

    fn spawn_task(_task: impl std::future::Future<Output = ()> + 'static) {
        todo!();
    }
}

fn reset_terminal() -> anyhow::Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct TuiCursor {
    location: Location,
    mode: Mode,
}

#[derive(Clone, Debug)]
enum Location {
    Detached,
    Node(NodeRef),
    EndOfChildren(NodeRef),
}

#[derive(Clone, Debug)]
enum Mode {
    Append,
    Diff,
}

impl TuiCursor {
    fn new_detached() -> Self {
        Self {
            location: Location::Detached,
            mode: Mode::Append,
        }
    }

    fn new_root() -> (Self, NodeRef) {
        let root = NodeRef(Rc::new(RefCell::new(Node {
            id: new_node_id(),
            kind: NodeKind::Empty,
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

    fn set_component(&mut self, component: Rc<ComponentData>) {
        self.set_node(NodeKind::Component(component));
    }

    fn set_node(&mut self, kind: NodeKind) {
        match (&self.mode, &self.location) {
            (Mode::Append, Location::Detached) => {
                let node = Rc::new(RefCell::new(Node {
                    id: new_node_id(),
                    kind,
                    parent: None,
                    first_child: None,
                    next_sibling: None,
                }));

                self.location = Location::Node(NodeRef(node));
            }
            (Mode::Append, Location::Node(node)) => {
                node.append_sibling(kind);
                self.location = Location::Node(node.next_sibling().unwrap());
            }
            (Mode::Append, Location::EndOfChildren(parent)) => {
                let node = Rc::new(RefCell::new(Node {
                    id: new_node_id(),
                    kind,
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
            other => todo!("{other:?}"),
        }
    }

    fn current_node(&self) -> NodeRef {
        match &self.location {
            Location::Node(node) => node.clone(),
            _ => panic!(),
        }
    }
}

impl autostrata::platform::Cursor for TuiCursor {
    type TextHandle = NodeRef;
    type EventHandle = ();

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
                match &mut borrow.kind {
                    NodeKind::Text(text) => {
                        *text = new_text.into();
                    }
                    _ => {}
                }
            }
            _ => panic!(),
        }
    }

    fn on_event(&mut self, _event: autostrata::On) -> () {}

    fn enter_children(&mut self) {
        match &self.location {
            Location::Node(node) => {
                self.location = match node.first_child() {
                    Some(first_child) => Location::Node(first_child),
                    None => Location::EndOfChildren(node.clone()),
                }
            }
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
            _ => panic!(),
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
            _ => panic!(),
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
        let mut replacement_cursor = Self::new_detached();
        func(&mut replacement_cursor);

        let Location::Node(node) = replacement_cursor.location else {
            panic!();
        };

        let kind = node.0.borrow().kind.clone();

        match &self.location {
            Location::Node(node) => {
                node.0.borrow_mut().kind = kind;
            }
            _ => panic!(),
        }
    }
}
