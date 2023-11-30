use kano::{Click, On};

pub struct TuiState {
    pub currently_focused: usize,
    pub focusable_counter: usize,
    pub focused_event_handler: Option<On<Click>>,
}

impl TuiState {
    pub fn on_pre_frame(&mut self) {
        self.focusable_counter = 0;
        self.focused_event_handler = None;
    }
}
