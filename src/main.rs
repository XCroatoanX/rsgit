mod app;
mod git;
mod startup;
mod ui;

use crate::startup::{StartupAction, ensure_git_repo};
use app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;
use std::env;
use std::time::Duration;
use ui::DashboardApp;

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
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(DashboardApp { app: &app }, frame.area());
        })?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                app.handle_key_event(key);
                if app.should_quit {
                    break;
                }
            }
        } else {
            app.tick();
        }
    }
    Ok(())
}
