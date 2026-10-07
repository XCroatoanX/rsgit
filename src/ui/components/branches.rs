use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState, Widget},
};

use crate::app::App;
use crate::ui::main_page::{ActiveBlock, BranchTab, create_block};

pub struct BranchesComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for BranchesComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::Branches);

        let items: Vec<ListItem> = match self.app.branch_tab {
            BranchTab::Local => self
                .app
                .git_data
                .local_branches
                .iter()
                .map(|b| ListItem::new(b.display_name()))
                .collect(),
            BranchTab::Remote => self
                .app
                .git_data
                .remote_branches
                .iter()
                .map(|b| ListItem::new(b.as_str()))
                .collect(),
            BranchTab::Tags => self
                .app
                .git_data
                .tags
                .iter()
                .map(|t| ListItem::new(t.as_str()))
                .collect(),
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let mut state = ListState::default();
        state.select(Some(self.app.selected_branch_index));

        ratatui::widgets::StatefulWidget::render(list, area, buf, &mut state);
    }
}
