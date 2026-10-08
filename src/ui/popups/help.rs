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
        Line::from(Span::styled("Navigation", BOLD)),
        Line::from(" [1-5]    Jump to Pane"),
        Line::from(" [Tab/h]   Next Pane"),
        Line::from(" [S-Tab/l] Previous Pane"),
        Line::from(" [Down/j]  Move Selection Down"),
        Line::from(" [Up/k]    Move Selection Up"),
        Line::from(" [[/]]     Switch Branch Tabs"),
        Line::from(""),
        Line::from(Span::styled("Branch / Tag Actions", BOLD)),
        Line::from(" [Enter]   Checkout Selected Target"),
        Line::from(" [n]       Create New Branch / Tag"),
        Line::from(" [r]       Rename Selected Item"),
        Line::from(" [d]       Delete Selected Item"),
        Line::from(" [p/P]     Pull / Push Selected Branch"),
        Line::from(""),
        Line::from(Span::styled("Popup Actions", BOLD)),
        Line::from(" [Enter]   Submit / Confirm"),
        Line::from(" [Esc]     Cancel / Close"),
        Line::from(" [Backspace] Delete Input Character"),
        Line::from(" [y/n]     Confirm / Cancel Delete"),
        Line::from(""),
        Line::from(Span::styled("General", BOLD)),
        Line::from(" [?]       Toggle Help"),
        Line::from(" [a]       Toggle About"),
        Line::from(" [q/Esc]   Close Popup / Quit"),
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
