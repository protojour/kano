use kano::attr::{Click, On};

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

    pub fn on_post_frame(&mut self) {
        if self.currently_focused >= self.focusable_counter {
            self.currently_focused = 0;
        }
    }
}
