use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::ActiveBlock;

pub struct CommitHistoryComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for CommitHistoryComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = self.app.create_block(ActiveBlock::CommitHistory);

        Paragraph::new("Commit history")
            .block(block)
            .render(area, buf);
    }
}
