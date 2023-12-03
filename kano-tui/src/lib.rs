//! Kano is a work-in-progress GUI application framework written for and in Rust.
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use kano::{platform::PlatformContext, vdom::vnode::VNodeRef};
use node_data::{NodeData, NodeKind};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
    Frame,
};
use std::{
    io::{self, stdout},
    panic,
    rc::Rc,
};
use tui_cursor::TuiCursor;
use tui_state::TuiState;

pub mod component;
pub mod node_data;

pub use ratatui;

mod tui_cursor;
mod tui_state;

pub struct Tui;

impl kano::platform::Platform for Tui {
    type Cursor = TuiCursor;

    fn init(signal_dispatch: Box<dyn Fn()>) -> PlatformContext {
        kano::history::push("".to_string());
        PlatformContext {
            on_signal_tick: Rc::new(|| {}),
            signal_dispatch,
            logger: Rc::new(|_| {}),
        }
    }

    fn run(view: impl kano::View<Self>, context: PlatformContext) -> anyhow::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            reset_terminal().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        let (mut cursor, root_node) = TuiCursor::new_root();
        let state = view.init(&mut cursor);
        std::mem::forget(state);

        let mut tui_state = TuiState {
            currently_focused: 0,
            focusable_counter: 0,
            focused_event_handler: None,
        };

        loop {
            tui_state.on_pre_frame();

            terminal.draw(|frame| {
                let area = frame.size();
                let view = root_node.first_child().unwrap();
                render_node(view, &mut tui_state, frame, area);
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            KeyCode::Up => {
                                if tui_state.currently_focused > 0 {
                                    tui_state.currently_focused -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if tui_state.currently_focused < tui_state.focusable_counter - 1 {
                                    tui_state.currently_focused += 1;
                                }
                            }
                            KeyCode::Char(' ') | KeyCode::Enter => {
                                if let Some(handler) = tui_state.focused_event_handler.take() {
                                    handler.invoke();
                                    (context.signal_dispatch)();
                                }
                            }
                            KeyCode::Backspace => {
                                if kano::history::pop() {
                                    (context.signal_dispatch)();
                                }
                            }
                            _ => {}
                        }
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

pub fn render_node(
    node: VNodeRef<NodeData>,
    tui_state: &mut TuiState,
    frame: &mut Frame,
    area: ratatui::prelude::Rect,
) {
    let borrow = node.0.borrow();
    match &borrow.data.kind {
        NodeKind::Empty => {}
        NodeKind::Text(text) => {
            frame.render_widget(Paragraph::new(text.as_str()), area);
        }
        NodeKind::Component(data) => {
            data.render(node.clone(), tui_state, frame, area);
        }
    }
}
