use crate::node::Node;

#[derive(Debug)]
pub enum Widget {}

impl Widget {
    pub fn render(
        &self,
        node: &Node,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
    }
}
