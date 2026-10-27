use crate::app::App;
use crate::state::Popup;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.popup = Popup::None,
        KeyCode::Char('?') => {
            app.popup = if app.popup == Popup::Help {
                Popup::None
            } else {
                Popup::Help
            };
        }
        KeyCode::Char('a') => {
            app.popup = if app.popup == Popup::About {
                Popup::None
            } else {
                Popup::About
            };
        }
        _ => {}
    }
}
