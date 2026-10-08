use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::ui::popups::get_dynamic_popup_area;

const BOLD_CYAN: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const DARK_GRAY: Style = Style::new().fg(Color::DarkGray);
const YELLOW: Style = Style::new().fg(Color::Yellow);

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = concat!(" v", env!("CARGO_PKG_VERSION"));
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn render_about_popup(area: Rect, buf: &mut Buffer) {
    let about_text = [
        Line::from(vec![
            Span::styled(APP_NAME, BOLD_CYAN),
            Span::raw(APP_VERSION),
        ]),
        Line::from(APP_DESCRIPTION),
        Line::from(""),
        Line::from(vec![
            Span::styled("Author: ", DARK_GRAY),
            Span::raw(APP_AUTHORS),
        ]),
        Line::from(vec![
            Span::styled("Repository: ", DARK_GRAY),
            Span::raw(APP_REPOSITORY),
        ]),
        Line::from(vec![Span::styled("License: ", DARK_GRAY), Span::raw("MIT")]),
        Line::from(""),
        Line::from(Span::styled("Press [Esc] or [a] to close", YELLOW)),
    ];

    let centered_area = get_dynamic_popup_area(area, &about_text, 4, 2);

    let about_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" About rsgit ");

    Clear.render(centered_area, buf);

    Paragraph::new(&about_text[..])
        .block(about_block)
        .alignment(Alignment::Center)
        .render(centered_area, buf);
}
