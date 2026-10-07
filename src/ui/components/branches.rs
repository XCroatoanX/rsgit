use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};
use strum::IntoEnumIterator;

use crate::app::App;
use crate::ui::{ActiveBlock, BranchTab, main_page::create_block_with_title};

pub struct BranchesComponent<'a> {
    pub app: &'a App,
}

impl<'a> BranchesComponent<'a> {
    fn format_title(&self) -> Line<'static> {
        let active_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let inactive_style = Style::default().fg(Color::DarkGray);

        let mut spans = vec![Span::raw("[2] Branches ( ")];

        for (i, tab) in BranchTab::iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(" | "));
            }
            let style = if self.app.branch_tab == tab {
                active_style
            } else {
                inactive_style
            };
            let text = if self.app.branch_tab == tab {
                format!("[{}]", tab.label())
            } else {
                tab.label().to_string()
            };
            spans.push(Span::styled(text, style));
        }

        spans.push(Span::raw(" )"));
        Line::from(spans)
    }
}

impl<'a> Widget for BranchesComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = self.format_title();
        let block = create_block_with_title(self.app, title, ActiveBlock::Branches);

        let branch_content = match self.app.branch_tab {
            BranchTab::Local => "Local Branches: main*, feature/ui",
            BranchTab::Remote => "Remote Branches: origin/main, origin/feature/ui",
            BranchTab::Tags => "Tags: v0.1.0, v0.0.1",
        };

        Paragraph::new(branch_content)
            .block(block)
            .render(area, buf);
    }
}
