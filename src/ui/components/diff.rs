use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::ActiveBlock;

pub struct DiffComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for DiffComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.app.create_block(ActiveBlock::Diff);

        Paragraph::new("Diff")
            .block(block)
            .render(area, buf);
    }
}
