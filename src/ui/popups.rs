pub mod about;
pub mod auth;
pub mod commit;
pub mod confirm;
pub mod error;
pub mod help;
pub mod selection;
pub mod textinput;

pub use about::render_about_popup;
pub use help::render_help_popup;

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders},
};

fn get_dynamic_popup_area(area: Rect, lines: &[Line], padding_x: u16, padding_y: u16) -> Rect {
    let max_text_width = lines
        .iter()
        .map(|line| line.width() as u16)
        .max()
        .unwrap_or(0);

    let content_width = (max_text_width + padding_x + 2).min(area.width);
    let content_height = ((lines.len() as u16) + padding_y + 2).min(area.height);

    area.centered(
        Constraint::Length(content_width),
        Constraint::Length(content_height),
    )
}

pub(crate) fn centered_rect(width_chars: u16, height_lines: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(width_chars)) / 2;
    let y = r.y + (r.height.saturating_sub(height_lines)) / 2;
    Rect::new(x, y, width_chars.min(r.width), height_lines.min(r.height))
}

pub(crate) fn popup_block(title: Line<'static>, border_color: Color) -> Block<'static> {
    Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(Color::Reset))
}
