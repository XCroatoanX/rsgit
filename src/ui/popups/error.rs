use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget, Wrap},
};

pub struct ErrorPopup<'a> {
    pub message: &'a str,
}

impl<'a> Widget for ErrorPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(65, 12, area);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(" Error ")
            .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Red))
            .style(Style::default().bg(Color::Reset));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(inner);

        Paragraph::new(self.message)
            .style(Style::default().fg(Color::LightRed))
            .wrap(Wrap { trim: true })
            .render(chunks[0], buf);

        Paragraph::new("[Esc / Enter] Dismiss")
            .style(Style::default().fg(Color::DarkGray))
            .render(chunks[1], buf);
    }
}

fn centered_rect(width_chars: u16, height_lines: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width_chars)) / 2;
    let y = r.y + (r.height.saturating_sub(height_lines)) / 2;
    Rect::new(x, y, width_chars.min(r.width), height_lines.min(r.height))
}