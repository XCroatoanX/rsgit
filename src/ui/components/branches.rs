use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Widget},
};

use crate::app::{App, SyncAction};
use crate::ui::main_page::{ActiveBlock, BranchTab, create_block};

const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct BranchesComponent<'a> {
    pub app: &'a App,
}

impl<'a> BranchesComponent<'a> {
    fn create_item_line(&self, name: &str, display_text: &str) -> ListItem<'a> {
        let is_syncing = self
            .app
            .active_sync
            .as_ref()
            .is_some_and(|sync| sync.branch_name == name);

        if is_syncing {
            let sync = self.app.active_sync.as_ref().unwrap();
            let frame = SPINNER[self.app.tick_count % SPINNER.len()];

            let (action_str, action_color) = match sync.action {
                SyncAction::Pull => (" pulling...", Color::Cyan),
                SyncAction::Push => (" pushing...", Color::Magenta),
            };

            let line = Line::from(vec![
                Span::raw(display_text.to_string()),
                Span::styled(format!(" {frame}"), Style::default().fg(Color::Yellow)),
                Span::styled(
                    action_str,
                    Style::default()
                        .fg(action_color)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]);

            ListItem::new(line)
        } else {
            ListItem::new(display_text.to_string())
        }
    }
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
                .map(|b| self.create_item_line(&b.name, &b.display_name()))
                .collect(),

            BranchTab::Remote => self
                .app
                .git_data
                .remote_branches
                .iter()
                .map(|b| self.create_item_line(b, b.as_str()))
                .collect(),

            BranchTab::Tags => self
                .app
                .git_data
                .tags
                .iter()
                .map(|t| self.create_item_line(t, t.as_str()))
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
