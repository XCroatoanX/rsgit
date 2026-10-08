use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.show_help = false;
            app.show_about = false;
        }
        KeyCode::Char('?') => {
            app.show_about = false;
            app.show_help = !app.show_help;
        }
        KeyCode::Char('a') => {
            app.show_help = false;
            app.show_about = !app.show_about;
        }
        _ => {}
    }
}
