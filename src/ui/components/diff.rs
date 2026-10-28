use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Widget},
};

use crate::app::App;
use crate::state::{ActiveBlock, DiffContext};
use crate::ui::components::common::commit_item;
use crate::ui::main_page::create_block_with_title;

pub struct DiffComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for DiffComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let context = match self.app.active_block {
            ActiveBlock::Files => DiffContext::Files,
            ActiveBlock::Branches => DiffContext::Branches,
            ActiveBlock::CommitHistory => DiffContext::CommitHistory,
            ActiveBlock::Diff | ActiveBlock::Logs => self.app.diff_context,
        };

        let title = match context {
            DiffContext::Files => "Diff",
            DiffContext::Branches => "Log",
            DiffContext::CommitHistory => "Commit diff",
        };
        let block = create_block_with_title(
            self.app,
            Line::from(format!("[{}] {}", ActiveBlock::Diff.index(), title)),
            ActiveBlock::Diff,
        );

        let items = match context {
            DiffContext::Files => match self.app.selected_file_path() {
                Some(path) if self.app.file_diff.is_empty() => {
                    vec![ListItem::new(format!("No diff available for {path}"))]
                }
                Some(_) => self
                    .app
                    .file_diff
                    .iter()
                    .map(|line| diff_item(line))
                    .collect(),
                None => vec![ListItem::new("No files selected")],
            },
            DiffContext::Branches => {
                if let Some(branch) = self.app.get_selected_entity_name() {
                    if self.app.branch_history.is_empty() {
                        vec![ListItem::new(format!("No commits found for {branch}"))]
                    } else {
                        self.app.branch_history.iter().map(commit_item).collect()
                    }
                } else {
                    vec![ListItem::new("No branch selected")]
                }
            }
            DiffContext::CommitHistory => {
                if self.app.commit_diff.is_empty() {
                    vec![ListItem::new("No changes found for the selected commit")]
                } else {
                    self.app
                        .commit_diff
                        .iter()
                        .map(|line| diff_item(line))
                        .collect()
                }
            }
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        let mut state = ListState::default();
        match context {
            DiffContext::Files => {
                if !self.app.file_diff.is_empty() {
                    state.select(Some(
                        self.app
                            .selected_file_diff_index
                            .min(self.app.file_diff.len().saturating_sub(1)),
                    ));
                }
            }
            DiffContext::Branches => {
                if !self.app.branch_history.is_empty() {
                    state.select(Some(self.app.selected_branch_history_index));
                }
            }
            DiffContext::CommitHistory => {
                if !self.app.commit_diff.is_empty() {
                    state.select(Some(
                        self.app
                            .selected_diff_index
                            .min(self.app.commit_diff.len().saturating_sub(1)),
                    ));
                }
            }
        }
        ratatui::widgets::StatefulWidget::render(list, area, buf, &mut state);
    }
}

fn diff_item(line: &str) -> ListItem<'static> {
    let color = if line.starts_with("+++") || line.starts_with("---") {
        Color::Yellow
    } else if line.starts_with('+') {
        Color::Green
    } else if line.starts_with('-') {
        Color::Red
    } else if line.starts_with("@@") {
        Color::Cyan
    } else if line.starts_with("diff ") || line.starts_with("index ") {
        Color::Magenta
    } else {
        Color::White
    };

    ListItem::new(Span::styled(line.to_string(), Style::default().fg(color)))
}
