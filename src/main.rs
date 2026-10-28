mod app;
mod events;
mod git;
mod popups;
mod startup;
mod state;
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
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::DisableBracketedPaste);
    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let _ = crossterm::execute!(std::io::stdout(), crossterm::event::EnableBracketedPaste);
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(DashboardApp { app: &app }, frame.area());
        })?;
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    events::handle_key_event(&mut app, key);
                    if app.should_quit {
                        break;
                    }
                }
                Event::Paste(text) => {
                    app.paste_text(&text);
                }
                _ => {}
            }
        } else {
            app.tick();
        }
    }
    Ok(())
}
