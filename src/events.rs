pub mod navigation;
pub mod overlays;
pub mod popups;

use crate::app::App;
use crossterm::event::KeyEvent;

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    if app.error_message.is_some() {
        if matches!(
            key.code,
            crossterm::event::KeyCode::Esc
                | crossterm::event::KeyCode::Enter
                | crossterm::event::KeyCode::Char('q')
        ) {
            app.error_message = None;
        }
        return;
    }

    if app.show_help || app.show_about {
        overlays::handle_keys(app, key);
        return;
    }

    if app.show_create_popup {
        popups::handle_create_keys(app, key);
        return;
    }
    if app.show_rename_popup {
        popups::handle_rename_keys(app, key);
        return;
    }
    if app.show_delete_popup {
        popups::handle_delete_keys(app, key);
        return;
    }

    navigation::handle_global_keys(app, key);
}
