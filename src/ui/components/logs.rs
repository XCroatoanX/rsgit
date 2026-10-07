use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::ActiveBlock;

pub struct LogsComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for LogsComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.app.create_block(ActiveBlock::Logs);

        Paragraph::new("Logs")
            .block(block)
            .render(area, buf);
    }
}
