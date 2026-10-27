use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Clear, Paragraph, Widget, Wrap},
};

use super::{centered_rect, popup_block};

pub struct ErrorPopup<'a> {
    pub message: &'a str,
}

impl<'a> Widget for ErrorPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(65, 12, area);

        Clear.render(popup_area, buf);

        let block = popup_block(Line::from(" Error "), Color::Red);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
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
