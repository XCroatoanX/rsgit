mod ui;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use ui::DashboardApp;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut show_help = false;
    let mut show_about = false;

    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                DashboardApp {
                    show_help,
                    show_about,
                },
                frame.area(),
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        if show_help {
                            show_help = false;
                        } else if show_about {
                            show_about = false;
                        } else {
                            break;
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('?') => {
                        show_about = false;
                        show_help = !show_help
                    }
                    KeyCode::Char('a') => {
                        show_help = false;
                        show_about = !show_about
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
