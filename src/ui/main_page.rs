use crate::app::App;
use crate::ui::popups::{render_about_popup, render_help_popup};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, EnumCount)]
pub enum ActiveBlock {
    #[default]
    StagedFiles,
    Branches,
    CommitHistory,
    Diff,
    Logs,
}

impl ActiveBlock {
    pub fn title(self) -> &'static str {
        match self {
            Self::StagedFiles => "Staged files",
            Self::Branches => "Branches",
            Self::CommitHistory => "Commit history",
            Self::Diff => "Diff",
            Self::Logs => "Git Logs",
        }
    }

    pub fn next(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn previous(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::iter().nth(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum BranchTab {
    #[default]
    Local,
    Remote,
    Tags,
}

impl BranchTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Remote => "Remote",
            Self::Tags => "Tags",
        }
    }

    pub fn next(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn previous(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

pub struct DashboardApp<'a> {
    pub app: &'a App,
}

impl<'a> DashboardApp<'a> {
    fn create_block(&self, title: Line<'static>, block_type: ActiveBlock) -> Block<'static> {
        let is_active = self.app.active_block == block_type;
        let border_color = if is_active {
            Color::LightGreen
        } else {
            Color::DarkGray
        };

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .title(title)
    }

    fn format_branch_title(&self) -> Line<'static> {
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

    fn render_pane(&self, area: Rect, buf: &mut Buffer, block_type: ActiveBlock, content: &str) {
        let index = ActiveBlock::iter()
            .position(|b| b == block_type)
            .unwrap_or(0)
            + 1;
        let title = Line::from(format!("[{}] {}", index, block_type.title()));
        Paragraph::new(content)
            .block(self.create_block(title, block_type))
            .render(area, buf);
    }
}

impl<'a> Widget for DashboardApp<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, shortcut_area] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(area);

        let [left_col, right_col] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        let [left_1, left_2, left_3] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(left_col);

        let [right_1, right_2] =
            Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                .areas(right_col);

        self.render_pane(left_1, buf, ActiveBlock::StagedFiles, "Staged files");

        let branch_content = match self.app.branch_tab {
            BranchTab::Local => "Local Branches: main*, feature/ui",
            BranchTab::Remote => "Remote Branches: origin/main, origin/feature/ui",
            BranchTab::Tags => "Tags: v0.1.0, v0.0.1",
        };

        Paragraph::new(branch_content)
            .block(self.create_block(self.format_branch_title(), ActiveBlock::Branches))
            .render(left_2, buf);

        self.render_pane(left_3, buf, ActiveBlock::CommitHistory, "Commit history");
        self.render_pane(right_1, buf, ActiveBlock::Diff, "Diff");
        self.render_pane(right_2, buf, ActiveBlock::Logs, "Logs");

        // Footer shortcuts bar
        Paragraph::new("[q] Quit | [?] Help | [a] About").render(shortcut_area, buf);

        if self.app.show_help {
            render_help_popup(area, buf);
        } else if self.app.show_about {
            render_about_popup(area, buf);
        }
    }
}
