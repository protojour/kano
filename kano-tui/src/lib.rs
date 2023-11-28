//! Kano is a work-in-progress GUI application framework written for and in Rust.
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use cursor::TuiCursor;
use kano::Diff;
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    io::{self, stdout},
    panic,
};
use tui_state::TuiState;

pub mod component;
pub mod node;

pub use ratatui;

mod cursor;
mod tui_state;

pub struct Tui;

impl kano::platform::Platform for Tui {
    type Cursor = TuiCursor;

    fn log(_s: &str) {}

    fn run_app<V: kano::View<Self>, F: (FnOnce() -> V) + 'static>(func: F) -> anyhow::Result<()> {
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
        let state = kano::view::Func(func, ()).init(&mut cursor);
        std::mem::forget(state);

        let root_node = empty_root.first_child().unwrap();

        let mut tui_state = TuiState {
            currently_focused: 0,
            focusable_counter: 0,
            focused_event_handler: None,
        };

        loop {
            tui_state.on_pre_frame();

            terminal.draw(|frame| {
                let area = frame.size();
                root_node.clone().render(&mut tui_state, frame, area);
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
                                    kano::dispatch_pending_signals();
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
