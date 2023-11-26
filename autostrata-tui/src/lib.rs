use crossterm::{
    event::{self, DisableMouseCapture, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use node::{Node, NodeKind, NodeRef};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::{
    cell::RefCell,
    io::{self, stdout},
    panic,
    rc::Rc,
};

pub mod node;
pub mod widget;

pub struct Tui;

impl autostrata::platform::Platform for Tui {
    type Cursor = TuiCursor;

    fn log(_s: &str) {}

    fn run_app<V: autostrata::View<Self>, F: (FnOnce() -> V) + 'static>(
        _func: F,
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

        loop {
            terminal.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
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
}

#[derive(Clone, Debug)]
enum Location {
    Detached,
    Node(Rc<RefCell<Node>>),
}

impl TuiCursor {
    fn set_node(&mut self, kind: NodeKind) {}
}

impl autostrata::platform::Cursor for TuiCursor {
    type TextHandle = NodeRef;
    type EventHandle = ();

    fn from_text_handle(handle: &Self::TextHandle) -> Self {
        todo!()
    }

    fn empty(&mut self) {
        self.set_node(NodeKind::Empty);
    }

    fn text(&mut self, text: &str) -> Self::TextHandle {
        self.set_node(NodeKind::Text(text.into()));
        todo!();
    }

    fn update_text(&mut self, _text: &str) {
        todo!()
    }

    fn on_event(&mut self, _event: autostrata::On) -> () {}

    fn enter_children(&mut self) {
        todo!()
    }

    fn exit_children(&mut self) {
        todo!()
    }

    fn next_sibling(&mut self) {
        todo!()
    }

    fn remove(&mut self) {
        todo!()
    }

    fn enter_diff(&mut self) {
        todo!()
    }

    fn exit_diff(&mut self) {
        todo!()
    }

    fn replace(&mut self, _func: impl FnOnce(&mut Self)) {
        todo!()
    }
}
