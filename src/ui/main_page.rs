use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::app::App;
use crate::ui::components::*;
use crate::ui::popups::{render_about_popup, render_help_popup};

pub fn create_block(app: &App, block_type: ActiveBlock) -> Block<'static> {
    let title = Line::from(format!("[{}] {}", block_type.index(), block_type.title()));
    create_block_with_title(app, title, block_type)
}

pub fn create_block_with_title(
    app: &App,
    title: Line<'static>,
    block_type: ActiveBlock,
) -> Block<'static> {
    let border_color = if app.active_block == block_type {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, EnumCount)]
pub enum ActiveBlock {
    #[default]
    Files,
    Branches,
    CommitHistory,
    Diff,
    Logs,
}

impl ActiveBlock {
    pub fn title(self) -> &'static str {
        match self {
            Self::Files => "Files",
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

    pub fn index(self) -> usize {
        Self::iter().position(|b| b == self).unwrap_or(0) + 1
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

        FilesComponent { app: self.app }.render(left_1, buf);
        BranchesComponent { app: self.app }.render(left_2, buf);
        CommitHistoryComponent { app: self.app }.render(left_3, buf);
        DiffComponent { app: self.app }.render(right_1, buf);
        LogsComponent { app: self.app }.render(right_2, buf);
        FooterComponent.render(shortcut_area, buf);

        if self.app.show_help {
            render_help_popup(area, buf);
        } else if self.app.show_about {
            render_about_popup(area, buf);
        }
    }
}
