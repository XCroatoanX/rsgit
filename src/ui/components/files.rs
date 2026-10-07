use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::ActiveBlock;

pub struct FilesComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for FilesComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.app.create_block(ActiveBlock::Files);

        Paragraph::new("Files").block(block).render(area, buf);
    }
}
