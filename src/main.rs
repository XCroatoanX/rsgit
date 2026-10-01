mod app;
mod ui;
mod startup;

use std::env;
use app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use ui::DashboardApp;
use crate::startup::{ensure_git_repo, StartupAction};

fn main() -> Result<()> {
    color_eyre::install()?;

    let target_dir = env::args().nth(1);
    match ensure_git_repo(target_dir.as_deref())? {
        StartupAction::ExitSuccess => std::process::exit(0),
        StartupAction::Continue => {}
    }

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::default();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(DashboardApp { app: &app }, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key_event(key);
                if app.should_quit {
                    break;
                }
            }
        }
    }

    Ok(())
}