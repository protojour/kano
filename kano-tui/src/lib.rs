//! Kano is a work-in-progress GUI application framework written for and in Rust.
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use kano::{
    markup::Markup,
    platform::{PlatformContext, PlatformInit},
    vdom::vnode::VNodeRef,
};
use kano_svg::Svg1_1;
use node_data::{NodeData, NodeKind};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
    Frame,
};
use std::{
    fs::OpenOptions,
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

/// The TUI platform.
pub struct Tui;

/// The TUI "markup language".
pub struct Tml;

impl Markup<Tui> for Tml {
    type Cursor = TuiCursor;
}

impl Markup<Tui> for Svg1_1 {
    type Cursor = TuiCursor;
}

impl kano::platform::Platform for Tui {
    type Markup = Tml;

    fn init(init: PlatformInit) -> PlatformContext {
        PlatformContext {
            on_signal_tick: Rc::new(|| {}),
            signal_dispatch: init.signal_dispatch,
            logger: Rc::new(|line| {
                // FIXME: Don't reinvent logging?
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(".kano_tui_log.txt")
                    .unwrap();

                use std::io::Write;

                if let Err(e) = writeln!(file, "{line}") {
                    eprintln!("Couldn't log line: {}", e);
                }
            }),
            history_api: Rc::new(kano::history::HistoryState::new("".to_string())),
        }
    }

    fn run(view: impl kano::View<Self, Tml>, context: PlatformContext) -> anyhow::Result<()> {
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
        let state = view.init_const(&mut cursor);
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

            tui_state.on_post_frame();

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
                                if tui_state.focusable_counter > 0
                                    && tui_state.currently_focused < tui_state.focusable_counter - 1
                                {
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
