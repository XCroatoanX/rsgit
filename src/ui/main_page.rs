use ratatui::style::Modifier;
use ratatui::text::Span;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Widget},
};
use strum::IntoEnumIterator;

use crate::app::App;
use crate::state::{ActiveBlock, BranchTab, Popup};
use crate::ui::components::*;
use crate::ui::popups::textinput::TextInputPopup;
use crate::ui::popups::{render_about_popup, render_help_popup};

use crate::ui::popups::confirm::ConfirmPopup;
use crate::ui::popups::error::ErrorPopup;
use crate::ui::popups::selection::{SelectionOption, SelectionPopup};

pub fn create_block(app: &App, block_type: ActiveBlock) -> Block<'static> {
    let title = match block_type {
        ActiveBlock::Branches => {
            let mut spans = vec![Span::raw(format!(
                "[{}] {} ",
                block_type.index(),
                block_type.title()
            ))];

            for tab in BranchTab::iter() {
                let style = if app.branch_tab == tab {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                spans.push(Span::styled(format!("[{}] ", tab.label()), style));
            }

            Line::from(spans)
        }
        _ => Line::from(format!("[{}] {}", block_type.index(), block_type.title())),
    };

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
        FooterComponent { app: self.app }.render(shortcut_area, buf);

        if let Some(ref err_msg) = self.app.error_message {
            ErrorPopup { message: err_msg }.render(area, buf);
            return;
        }

        match self.app.popup {
            Popup::None => {}
            Popup::Delete => {
                let options: Vec<SelectionOption> = self
                    .app
                    .delete_options
                    .iter()
                    .map(|(_, label, enabled)| SelectionOption {
                        label,
                        enabled: *enabled,
                    })
                    .collect();

                let title = match self.app.branch_tab {
                    BranchTab::Local => "Delete Local Branch",
                    BranchTab::Remote => "Delete Remote Branch",
                    BranchTab::Tags => "Delete Tag",
                };

                SelectionPopup {
                    title,
                    options: &options,
                    selected_index: self.app.delete_selected_index,
                    warning: None,
                }
                .render(area, buf);
            }
            Popup::Create => {
                let (title, label) = match self.app.branch_tab {
                    BranchTab::Local => (
                        "Create Local Branch",
                        "Enter branch name (spaces convert to '-'):",
                    ),
                    BranchTab::Remote => (
                        "Create Remote Branch",
                        "Enter remote branch name (spaces convert to '-'):",
                    ),
                    BranchTab::Tags => ("Create Tag", "Enter tag name (spaces convert to '-'):"),
                };

                TextInputPopup {
                    title,
                    label,
                    input: &self.app.new_entity_input,
                    error: self.app.create_error_message.as_deref(),
                }
                .render(area, buf);
            }
            Popup::Rename => {
                let (title, label) = match self.app.branch_tab {
                    BranchTab::Local => ("Rename Local Branch", "Enter new branch name:"),
                    BranchTab::Remote => ("Rename Remote Branch", "Enter new remote branch name:"),
                    BranchTab::Tags => ("Rename Tag", "Enter new tag name:"),
                };

                TextInputPopup {
                    title,
                    label,
                    input: &self.app.rename_input,
                    error: self.app.rename_error_message.as_deref(),
                }
                .render(area, buf);
            }
            Popup::Help => render_help_popup(area, buf),
            Popup::About => render_about_popup(area, buf),
            Popup::Merge => {
                let options: Vec<SelectionOption> = self
                    .app
                    .merge_options
                    .iter()
                    .map(|(_, label, enabled)| SelectionOption {
                        label,
                        enabled: *enabled,
                    })
                    .collect();

                let title = self.app.merge_popup_title();
                SelectionPopup {
                    title: &title,
                    options: &options,
                    selected_index: self.app.merge_selected_index,
                    warning: None,
                }
                .render(area, buf);
            }
            Popup::Rebase => {
                let message = format!(
                    "Rebase '{}' onto '{}'?",
                    self.app.rebase_source, self.app.rebase_target
                );

                ConfirmPopup {
                    title: "Rebase",
                    message: &message,
                    error: None,
                }
                .render(area, buf);
            }
        }
    }
}
