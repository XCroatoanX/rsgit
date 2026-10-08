pub mod about;
pub mod confirm;
pub mod error;
pub mod help;
pub mod selection;
pub mod textinput;

pub use about::render_about_popup;
pub use help::render_help_popup;

use ratatui::{
    layout::{Constraint, Rect},
    text::Line,
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
