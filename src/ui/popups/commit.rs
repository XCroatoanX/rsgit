use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Widget},
};

use crate::state::CommitField;

use super::{centered_rect, popup_block};

pub struct CommitPopup<'a> {
    pub title: &'a str,
    pub commit_title: &'a str,
    pub commit_title_cursor: usize,
    pub commit_message: &'a str,
    pub commit_message_cursor: usize,
    pub active_field: CommitField,
    pub error: Option<&'a str>,
}

impl<'a> Widget for CommitPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let desc_lines = self.commit_message.matches('\n').count() + 1;
        let desc_field_h = (desc_lines as u16 + 2).min(8);
        let popup_h = (8 + desc_field_h).min(area.height);

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
                Constraint::Length(desc_field_h),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(padded[1]);

        Paragraph::new("Title")
            .style(Style::default().fg(Color::Gray))
            .render(chunks[0], buf);
        render_input(
            self.commit_title,
            self.active_field == CommitField::Title,
            chunks[1].width,
            self.commit_title_cursor,
        )
        .render(chunks[1], buf);

        Paragraph::new("Description")
            .style(Style::default().fg(Color::Gray))
            .render(chunks[2], buf);
        render_textarea(
            self.commit_message,
            self.active_field == CommitField::Message,
            chunks[3].width,
            chunks[3].height,
            self.commit_message_cursor,
        )
        .render(chunks[3], buf);

        if let Some(err) = self.error {
            Paragraph::new(format!("Error: {}", err))
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                .render(chunks[4], buf);
        } else {
            Paragraph::new("[Tab] Switch  |  [Enter] Commit/New Line  |  [Esc] Cancel")
                .style(Style::default().fg(Color::DarkGray))
                .render(chunks[4], buf);
        }
    }
}

fn input_block(active: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(if active {
            Color::Yellow
        } else {
            Color::Gray
        }))
}

fn render_input<'a>(
    value: &'a str,
    active: bool,
    field_width: u16,
    cursor: usize,
) -> Paragraph<'a> {
    let cursor_chars = cursor.min(value.chars().count());
    let content_width = field_width.saturating_sub(2) as usize;
    let scroll_x = (cursor_chars + 1).saturating_sub(content_width) as u16;

    let before: String = value.chars().take(cursor_chars).collect();
    let after: String = value.chars().skip(cursor_chars).collect();

    let cursor_span = Span::styled(
        " ",
        Style::default()
            .bg(if active { Color::Yellow } else { Color::DarkGray })
            .fg(Color::Black),
    );

    let input_spans = Line::from(vec![Span::raw(before), cursor_span, Span::raw(after)]);

    Paragraph::new(input_spans)
        .scroll((0, scroll_x))
        .block(input_block(active))
}

fn render_textarea<'a>(
    value: &'a str,
    active: bool,
    field_width: u16,
    field_height: u16,
    cursor: usize,
) -> Paragraph<'a> {
    let cursor = cursor.min(value.chars().count());
    let prefix: String = value.chars().take(cursor).collect();
    let cursor_line = prefix.chars().filter(|c| *c == '\n').count();
    let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let cursor_col = prefix[line_start..].chars().count();

    let cursor_span = Span::styled(
        " ",
        Style::default()
            .bg(if active { Color::Yellow } else { Color::DarkGray })
            .fg(Color::Black),
    );

    let mut lines: Vec<Line<'a>> = Vec::new();
    for (i, line) in value.lines().enumerate() {
        if i == cursor_line {
            let before: String = line.chars().take(cursor_col).collect();
            let after: String = line.chars().skip(cursor_col).collect();
            let mut rendered = Line::from(before);
            rendered.push_span(cursor_span.clone());
            rendered.push_span(Span::raw(after));
            lines.push(rendered);
        } else {
            lines.push(Line::from(line));
        }
    }
    if cursor_line >= lines.len() {
        lines.push(Line::from(cursor_span));
    }

    let content_height = field_height.saturating_sub(2) as usize;
    let scroll_y = (cursor_line + 1).saturating_sub(content_height) as u16;
    let content_width = field_width.saturating_sub(2) as usize;
    let scroll_x = (cursor_col + 1).saturating_sub(content_width) as u16;

    Paragraph::new(lines)
        .scroll((scroll_y, scroll_x))
        .block(input_block(active))
}