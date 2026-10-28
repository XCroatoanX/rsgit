use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Widget},
};

use crate::app::App;
use crate::git::ChangedFile;
use crate::git::files::DiffRow;
use crate::state::ActiveBlock;
use crate::ui::main_page::create_block;

pub struct FilesComponent<'a> {
    pub app: &'a App,
}

impl<'a> Widget for FilesComponent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = self.app.diff_rows();

        let items: Vec<ListItem> = rows
            .iter()
            .map(|row| {
                let expanded = match row {
                    DiffRow::Folder { path, .. } => !self.app.collapsed_folders.contains(path),
                    DiffRow::File { .. } => false,
                };
                row_item(row, expanded)
            })
            .collect();

        let total = rows.len();
        let counter = if total == 0 {
            "0/0".to_string()
        } else {
            format!(
                " {}/{} ",
                self.app.selected_file_index.min(total - 1) + 1,
                total
            )
        };

        let block = create_block(self.app, ActiveBlock::Files)
            .title_bottom(Line::from(counter).right_aligned());

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let mut state = ListState::default();
        if !rows.is_empty() {
            state.select(Some(self.app.selected_file_index));
        }
        ratatui::widgets::StatefulWidget::render(list, area, buf, &mut state);
    }
}

fn row_item(row: &DiffRow, expanded: bool) -> ListItem<'static> {
    match row {
        DiffRow::Folder {
            name,
            path: _,
            depth,
        } => {
            let marker = if expanded { "▾" } else { "▸" };
            let indent = "  ".repeat(*depth);
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{indent}{marker} "),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(name.clone(), Style::default().fg(Color::Cyan)),
            ]))
        }
        DiffRow::File { file, depth } => {
            let indent = "  ".repeat(*depth);
            let name = file.path.rsplit('/').next().unwrap_or_default();
            let mut spans = status_spans(file);
            spans.insert(0, Span::raw(indent));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                name.to_string(),
                Style::default().fg(filename_color(file)),
            ));
            ListItem::new(Line::from(spans))
        }
    }
}

fn status_spans(file: &ChangedFile) -> Vec<Span<'static>> {
    let index = file.index;
    let worktree = file.worktree;

    if index == '?' || index == '!' {
        return vec![Span::styled("?", Style::default().fg(Color::DarkGray))];
    }

    if index != ' ' && worktree != ' ' {
        return vec![
            Span::styled(index.to_string(), Style::default().fg(staged_color(index))),
            Span::styled(
                worktree.to_string(),
                Style::default().fg(status_color(worktree)),
            ),
        ];
    }

    if index != ' ' {
        return vec![Span::styled(
            index.to_string(),
            Style::default().fg(status_color(index)),
        )];
    }

    vec![Span::styled(
        worktree.to_string(),
        Style::default().fg(status_color(worktree)),
    )]
}

fn filename_color(file: &ChangedFile) -> Color {
    if file.index != ' ' && file.index != '?' {
        Color::Green
    } else {
        Color::Red
    }
}

const fn status_color(code: char) -> Color {
    match code {
        'A' => Color::Green,
        'D' => Color::Red,
        'M' => Color::Yellow,
        'R' | 'C' => Color::Cyan,
        'U' | 'T' => Color::Magenta,
        _ => Color::DarkGray,
    }
}

const fn staged_color(code: char) -> Color {
    match code {
        'D' => Color::Red,
        _ => Color::Green,
    }
}
