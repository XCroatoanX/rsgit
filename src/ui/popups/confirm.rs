use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use super::{centered_rect, popup_block};

pub struct ConfirmPopup<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub error: Option<&'a str>,
}

impl<'a> Widget for ConfirmPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(55, 9, area);

        Clear.render(popup_area, buf);

        let block = popup_block(Line::from(format!(" {} ", self.title)), Color::Yellow);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let constraints = if self.error.is_some() {
            vec![
                Constraint::Min(2),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
        } else {
            vec![Constraint::Min(2), Constraint::Length(1)]
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(inner);

        Paragraph::new(self.message)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .render(chunks[0], buf);

        if let Some(err) = self.error {
            Paragraph::new(err)
                .style(Style::default().fg(Color::Red))
                .wrap(Wrap { trim: true })
                .render(chunks[1], buf);

            Paragraph::new("[y] Confirm  |  [n / Esc] Cancel")
                .style(Style::default().fg(Color::DarkGray))
                .render(chunks[2], buf);
        } else {
            Paragraph::new("[y / Enter] Confirm  |  [n / Esc] Cancel")
                .style(Style::default().fg(Color::DarkGray))
                .render(chunks[1], buf);
        }
    }
}
