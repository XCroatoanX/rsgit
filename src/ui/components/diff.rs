use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::{ActiveBlock, main_page::create_block};

pub struct DiffComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for DiffComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::Diff);

        Paragraph::new("Diff").block(block).render(area, buf);
    }
}
