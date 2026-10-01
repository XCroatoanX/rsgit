use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

const BOLD_CYAN: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
const DARK_GRAY: Style = Style::new().fg(Color::DarkGray);
const YELLOW: Style = Style::new().fg(Color::Yellow);
const BOLD: Style = Style::new().add_modifier(Modifier::BOLD);

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = concat!(" v", env!("CARGO_PKG_VERSION"));
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const APP_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

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

pub fn render_help_popup(area: Rect, buf: &mut Buffer) {
    let help_text = [
        Line::from(Span::styled("Shortcuts", BOLD)),
        Line::from(""),
        Line::from(" [1-5]  Select Pane"),
        Line::from(" [Tab]   Next Pane"),
        Line::from(" [h/l]  Navigation between panes"),
        Line::from(" [[/]]  Navigation inside pane"),
        Line::from(" [?]   Toggle Help"),
        Line::from(" [a]   Toggle About"),
        Line::from(" [q]   Quit Application"),
        Line::from(" [Esc] Close Popups / Quit"),
    ];

    let centered_area = get_dynamic_popup_area(area, &help_text, 4, 2);

    let popup_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Help / Shortcuts ");

    Clear.render(centered_area, buf);

    Paragraph::new(&help_text[..])
        .block(popup_block)
        .render(centered_area, buf);
}

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
