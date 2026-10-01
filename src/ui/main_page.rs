use crate::ui::popups::{render_about_popup, render_help_popup};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

pub struct DashboardApp {
    pub show_help: bool,
    pub show_about: bool,
}

impl Widget for DashboardApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let DashboardApp { show_help, show_about } = self;

        let [main_area, shortcut_area] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
            .areas(area);

        let [left_col, right_col] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
            .areas(main_area);

        let [left_1, left_2, left_3] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
            .areas(left_col);

        let [right_1, right_2] = Layout::vertical([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
            .areas(right_col);

        Paragraph::new("Staged files")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Staged files"))
            .render(left_1, buf);

        Paragraph::new("Branches")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Branches"))
            .render(left_2, buf);

        Paragraph::new("Commit history")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Commit history"))
            .render(left_3, buf);

        Paragraph::new("Diff")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Diff"))
            .render(right_1, buf);

        Paragraph::new("Logs")
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Git Logs"))
            .render(right_2, buf);

        Paragraph::new(" [q] Quit | [?] Help | [a] About rsgit ")
            .render(shortcut_area, buf);

        if show_help {
            render_help_popup(area, buf);
        }

        if show_about {
            render_about_popup(area, buf);
        }
    }
}