use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::app::App;
use crate::ui::{ActiveBlock, main_page::create_block};

const CURSOR: &str = "> ";
const INDENT: &str = "  ";
const ICON_WIDTH: usize = 2;
const SELECTED_BG: Color = Color::Rgb(45, 45, 45);

pub struct LogsComponent<'a> {
    pub app: &'a App,
}

impl Widget for LogsComponent<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = create_block(self.app, ActiveBlock::Logs);

        let inner_width = (area.width as usize).saturating_sub(2);
        let inner_height = (area.height as usize).saturating_sub(2);
        let text_width = inner_width
            .saturating_sub(CURSOR.len())
            .saturating_sub(ICON_WIDTH)
            .max(1);

        let mut lines: Vec<Line<'static>> = Vec::new();
        let mut ranges: Vec<(usize, usize)> = Vec::new();
        let selected = if self.app.git_log.is_empty() {
            0
        } else {
            self.app.selected_log_index.min(self.app.git_log.len() - 1)
        };

        if self.app.git_log.is_empty() {
            lines.push(Line::styled(
                "No log entries yet",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            for (index, message) in self.app.git_log.iter().enumerate() {
                let start = lines.len();
                lines.extend(entry_lines(message, text_width, index == selected));
                ranges.push((start, lines.len()));
            }
        }

        let offset = scroll_offset(&ranges, selected, inner_height, lines.len());

        let counter = if ranges.is_empty() {
            "0/0".to_string()
        } else {
            let (start, end) = ranges[selected];
            let total = lines.len();
            if end - start > 1 {
                format!(" {}-{}/{} ", start + 1, end, total)
            } else {
                format!(" {}/{} ", start + 1, total)
            }
        };

        Paragraph::new(lines)
            .block(block.title_bottom(Line::from(counter).right_aligned()))
            .scroll((offset.min(u16::MAX as usize) as u16, 0))
            .render(area, buf);
    }
}

fn scroll_offset(
    ranges: &[(usize, usize)],
    selected: usize,
    height: usize,
    total_lines: usize,
) -> usize {
    if height == 0 || total_lines <= height || ranges.is_empty() {
        return 0;
    }

    let max_offset = total_lines - height;
    let (start, end) = ranges[selected.min(ranges.len() - 1)];
    let entry_height = end - start;

    let offset = if entry_height >= height {
        start
    } else {
        end.saturating_sub(height)
    };

    offset.min(max_offset)
}

fn entry_lines(message: &str, width: usize, selected: bool) -> Vec<Line<'static>> {
    let (icon, icon_color, message_color) = log_level(message);

    wrap_message(message, width)
        .into_iter()
        .enumerate()
        .map(|(index, part)| {
            let cursor = if selected && index == 0 {
                Span::styled(CURSOR, Style::default().fg(Color::Yellow))
            } else {
                Span::raw(INDENT)
            };

            let prefix = if index == 0 {
                Span::styled(
                    format!("{icon} "),
                    Style::default().fg(icon_color).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(INDENT)
            };

            let padding = width.saturating_sub(part.chars().count());
            let mut spans = vec![
                cursor,
                prefix,
                Span::styled(part, Style::default().fg(message_color)),
            ];
            if padding > 0 {
                spans.push(Span::raw(" ".repeat(padding)));
            }

            let line = Line::from(spans);
            if selected {
                line.style(Style::default().bg(SELECTED_BG))
            } else {
                line
            }
        })
        .collect()
}

fn log_level(message: &str) -> (&'static str, Color, Color) {
    if message.starts_with("Success:") || message.contains("succeeded") {
        ("✔", Color::Green, Color::White)
    } else if message.starts_with("Failed") || message.contains(" failed") {
        ("✖", Color::Red, Color::LightRed)
    } else if message.starts_with("Running:") {
        ("▶", Color::Cyan, Color::White)
    } else if message.starts_with("Starting") {
        ("…", Color::Blue, Color::White)
    } else {
        ("•", Color::DarkGray, Color::Gray)
    }
}

fn wrap_message(message: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![message.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for word in message.split(' ') {
        let word_width = word.chars().count();

        if word_width > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current_width = 0;
            }

            let chars: Vec<char> = word.chars().collect();
            let mut start = 0;
            while start < chars.len() {
                let end = (start + width).min(chars.len());
                let chunk: String = chars[start..end].iter().collect();
                if end == chars.len() {
                    current = chunk;
                    current_width = end - start;
                } else {
                    lines.push(chunk);
                }
                start = end;
            }
        } else if current.is_empty() {
            current.push_str(word);
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current.push(' ');
            current.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
            current_width = word_width;
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}
