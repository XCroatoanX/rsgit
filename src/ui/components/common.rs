use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use crate::git::CommitLine;

pub fn commit_item(commit: &CommitLine) -> ListItem<'static> {
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
