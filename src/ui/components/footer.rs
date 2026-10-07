use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

pub struct FooterComponent;

impl Widget for FooterComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("[q] Quit | [?] Help | [a] About").render(area, buf);
    }
}
