use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::state::AuthField;

use super::{centered_rect, popup_block};

pub struct AuthPopup<'a> {
    pub title: &'a str,
    pub remote_url: &'a str,
    pub username: &'a str,
    pub username_cursor: usize,
    pub password: &'a str,
    pub password_cursor: usize,
    pub active_field: AuthField,
    pub error: Option<&'a str>,
}

impl<'a> Widget for AuthPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_h = 14;
        let popup_area = centered_rect(60, popup_h, area);

        Clear.render(popup_area, buf);

        let border_color = if self.error.is_some() {
            Color::Red
        } else {
            Color::LightGreen
        };

        let block = popup_block(Line::from(format!(" {} ", self.title)), border_color);
        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let padded = Layout::default()
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
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(padded[1]);

        if self.remote_url.is_empty() {
            Paragraph::new("No remote 'origin' configured.")
                .style(Style::default().fg(Color::Gray))
                .render(chunks[0], buf);
        } else {
            Paragraph::new(Line::from(Span::styled(
                format!("Remote: {}", self.remote_url),
                Style::default().fg(Color::Gray),
            )))
            .render(chunks[0], buf);
        }

        render_field(
            self.username,
            self.active_field == AuthField::Username,
            chunks[1].width,
            self.username_cursor,
            false,
        )
        .render(chunks[1], buf);

        Paragraph::new("Password (masked)")
            .style(Style::default().fg(Color::Gray))
            .render(chunks[2], buf);

        render_field(
            self.password,
            self.active_field == AuthField::Password,
            chunks[3].width,
            self.password_cursor,
            true,
        )
        .render(chunks[3], buf);

        if let Some(err) = self.error {
            Paragraph::new(format!("Error: {}", err))
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .render(chunks[4], buf);
        } else {
            Paragraph::new(
                "[Tab] Switch  |  [Enter] Authenticate  |  [Ctrl+V] Paste  |  [Esc] Cancel",
            )
            .style(Style::default().fg(Color::DarkGray))
            .render(chunks[4], buf);
        }
    }
}

fn input_block(active: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(if active { Color::Yellow } else { Color::Gray }))
}

fn render_field<'a>(
    value: &'a str,
    active: bool,
    field_width: u16,
    cursor: usize,
    masked: bool,
) -> Paragraph<'a> {
    let chars: Vec<char> = value.chars().collect();
    let cursor_chars = cursor.min(chars.len());
    let content_width = field_width.saturating_sub(2) as usize;
    let scroll_x = (cursor_chars + 1).saturating_sub(content_width) as u16;

    let display: Vec<char> = if masked {
        vec!['•'; chars.len()]
    } else {
        chars
    };
    let before: String = display.iter().take(cursor_chars).collect();
    let after: String = display.iter().skip(cursor_chars).collect();

    let cursor_span = Span::styled(
        " ",
        Style::default()
            .bg(if active {
                Color::Yellow
            } else {
                Color::DarkGray
            })
            .fg(Color::Black),
    );

    Paragraph::new(Line::from(vec![
        Span::raw(before),
        cursor_span,
        Span::raw(after),
    ]))
    .scroll((0, scroll_x))
    .block(input_block(active))
}
