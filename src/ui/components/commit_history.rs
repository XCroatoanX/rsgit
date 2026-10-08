use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::{ActiveBlock, main_page::create_block};

pub struct CommitHistoryComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for CommitHistoryComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::CommitHistory);

        Paragraph::new("Commit history")
            .block(block)
            .render(area, buf);
    }
}
