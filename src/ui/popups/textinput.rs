use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use super::{centered_rect, popup_block};

pub struct TextInputPopup<'a> {
    pub title: &'a str,
    pub label: &'a str,
    pub input: &'a str,
    pub error: Option<&'a str>,
}

impl<'a> Widget for TextInputPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(60, 11, area);

        Clear.render(popup_area, buf);

        let border_color = if self.error.is_some() {
            Color::Red
        } else {
            Color::LightGreen
        };

        let block = popup_block(Line::from(format!(" {} ", self.title)), border_color);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let padded_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(inner);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(padded_chunks[1]);

        Paragraph::new(self.label)
            .style(Style::default().fg(Color::Gray))
            .render(chunks[0], buf);

        let input_spans = Line::from(vec![
            Span::raw(self.input),
            Span::styled(" ", Style::default().bg(Color::Yellow).fg(Color::Black)),
        ]);

        let input_widget = Paragraph::new(input_spans).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        input_widget.render(chunks[2], buf);

        if let Some(err) = self.error {
            let err_widget = Paragraph::new(format!("Error: {}", err))
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            err_widget.render(chunks[3], buf);
        } else {
            let hint = Paragraph::new("[Enter] Submit  |  [Esc] Cancel")
                .style(Style::default().fg(Color::DarkGray));
            hint.render(chunks[3], buf);
        }
    }
}
