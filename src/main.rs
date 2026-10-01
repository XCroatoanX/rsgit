mod app;
mod ui;

use app::App;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyEventKind};
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