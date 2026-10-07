use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};

use crate::ui::popups::get_dynamic_popup_area;

const BOLD: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn render_help_popup(area: Rect, buf: &mut Buffer) {
    let help_text = [
        Line::from(Span::styled("Shortcuts", BOLD)),
        Line::from(""),
        Line::from(" [1-5]  Select Pane"),
        Line::from(" [Tab]  Next Pane"),
        Line::from(" [h/l]  Navigation between panes"),
        Line::from(" [[/]]  Navigation inside pane"),
        Line::from(" [?]    Toggle Help"),
        Line::from(" [a]    Toggle About"),
        Line::from(" [q]    Quit Application"),
        Line::from(" [Esc]  Close Popups / Quit"),
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
