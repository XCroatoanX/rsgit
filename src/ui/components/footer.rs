use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{
    app::App,
    ui::main_page::{ActiveBlock, BranchTab},
};

pub struct FooterComponent<'a> {
    pub app: &'a App,
}

impl<'a> FooterComponent<'a> {
    fn branch_shortcuts(&self) -> Vec<Span<'static>> {
        let mut shortcuts = vec![Span::raw("[Enter] Checkout")];

        match self.app.branch_tab {
            BranchTab::Local => {
                shortcuts.push(Span::raw(" | [n] New"));
                shortcuts.push(Span::raw(" | [d] Remove"));
                shortcuts.push(Span::raw(" | [p] Pull"));
                shortcuts.push(Span::raw(" | [P] Push"));
            }
            BranchTab::Remote => {
                shortcuts.push(Span::raw(" | [n] New remote"));
                shortcuts.push(Span::raw(" | [d] Remove remote"));
            }
            BranchTab::Tags => {
                shortcuts.push(Span::raw(" | [n] New tag"));
                shortcuts.push(Span::raw(" | [d] Remove tag"));
            }
        }

        shortcuts
    }
}

impl<'a> Widget for FooterComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shortcuts = if self.app.active_block == ActiveBlock::Branches {
            let mut shortcuts = self.branch_shortcuts();
            shortcuts.push(Span::raw(" | [q] Quit | [?] Help | [a] About"));
            Line::from(shortcuts)
        } else {
            Line::from("[q] Quit | [?] Help | [a] About")
        };

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(20)]).areas(area);

        Paragraph::new(shortcuts).render(left_area, buf);
        Paragraph::new(Line::from(vec![
            Span::styled(
                env!("CARGO_PKG_NAME"),
                Style::default()
                    .fg(Color::Rgb(222, 97, 45))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" v{}", env!("CARGO_PKG_VERSION"))),
        ]))
        .right_aligned()
        .render(right_area, buf);
    }
}
