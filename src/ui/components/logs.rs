use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::{ActiveBlock, main_page::create_block};

pub struct LogsComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for LogsComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::Logs);

        Paragraph::new("Logs").block(block).render(area, buf);
    }
}
