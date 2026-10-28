pub mod navigation;
pub mod overlays;
pub mod popups;

use crate::app::App;
use crate::state::Popup;
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

    match app.popup {
        Popup::Help | Popup::About => overlays::handle_keys(app, key),
        Popup::Create => popups::handle_create_keys(app, key),
        Popup::Rename => popups::handle_rename_keys(app, key),
        Popup::Delete => popups::handle_delete_keys(app, key),
        Popup::Merge => popups::handle_merge_keys(app, key),
        Popup::Rebase => popups::handle_rebase_keys(app, key),
        Popup::Commit => popups::handle_commit_keys(app, key),
        Popup::Auth => popups::handle_auth_keys(app, key),
        Popup::None => navigation::handle_global_keys(app, key),
    }
}
