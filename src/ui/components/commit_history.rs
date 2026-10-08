use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Widget},
};

use crate::app::App;
use crate::git::CommitLine;
use crate::ui::{ActiveBlock, main_page::create_block};

pub struct CommitHistoryComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for CommitHistoryComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::CommitHistory);

        let items: Vec<ListItem> = if self.app.checked_out_history.is_empty() {
            vec![ListItem::new("No commits found for the checked out branch")]
        } else {
            self.app
                .checked_out_history
                .iter()
                .map(commit_item)
                .collect()
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        let mut state = ListState::default();
        if !self.app.checked_out_history.is_empty() {
            state.select(Some(self.app.selected_commit_index));
        }
        ratatui::widgets::StatefulWidget::render(list, area, buf, &mut state);
    }
}

fn commit_item(commit: &CommitLine) -> ListItem<'static> {
    let Some(hash) = &commit.hash else {
        return ListItem::new(Span::styled(
            commit.graph.clone(),
            Style::default().fg(Color::DarkGray),
        ));
    };

    ListItem::new(Line::from(vec![
        Span::styled(commit.graph.clone(), Style::default().fg(Color::DarkGray)),
        Span::styled(
            "● ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(hash.clone(), Style::default().fg(Color::Yellow)),
        Span::raw(" "),
        Span::styled(
            commit.author.clone().unwrap_or_default(),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled(
            commit.subject.clone().unwrap_or_default(),
            Style::default().fg(Color::White),
        ),
    ]))
}
