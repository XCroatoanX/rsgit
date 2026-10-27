use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

use super::{centered_rect, popup_block};

pub struct SelectionOption {
    pub label: &'static str,
    pub enabled: bool,
}

pub struct SelectionPopup<'a> {
    pub title: &'a str,
    pub options: &'a [SelectionOption],
    pub selected_index: usize,
    pub warning: Option<&'a str>,
}

impl<'a> Widget for SelectionPopup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = centered_rect(55, 11, area);
        Clear.render(popup_area, buf);

        let block = popup_block(Line::from(format!(" {} ", self.title)), Color::Yellow);

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(self.options.len() as u16 + 1),
                Constraint::Length(2),
                Constraint::Length(1),
            ])
            .split(inner);

        let mut lines = Vec::new();
        for (i, opt) in self.options.iter().enumerate() {
            let is_selected = i == self.selected_index;
            let prefix = if is_selected { "> " } else { "  " };

            let style = if !opt.enabled {
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(Color::Yellow)),
                Span::styled(opt.label, style),
            ]));
        }
        Paragraph::new(lines).render(chunks[0], buf);

        if let Some(warn) = self.warning {
            Paragraph::new(format!("⚠ {}", warn))
                .style(Style::default().fg(Color::LightRed))
                .render(chunks[1], buf);
        }

        Paragraph::new("[j/k or ↑/↓] Navigate  |  [Enter] Confirm  |  [Esc] Cancel")
            .style(Style::default().fg(Color::DarkGray))
            .render(chunks[2], buf);
    }
}
