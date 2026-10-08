use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

pub struct ConfirmPopup<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub error: Option<&'a str>,
}

impl<'a> Widget for ConfirmPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(55, 9, area);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .title_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow))
            .style(Style::default().bg(Color::Reset));

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

fn centered_rect(width_chars: u16, height_lines: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width_chars)) / 2;
    let y = r.y + (r.height.saturating_sub(height_lines)) / 2;
    Rect::new(x, y, width_chars.min(r.width), height_lines.min(r.height))
}
