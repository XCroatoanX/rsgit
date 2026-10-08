use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_create_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.submit_create_entity(),
        KeyCode::Esc => app.close_create_popup(),
        KeyCode::Backspace => {
            app.new_entity_input.pop();
        }
        KeyCode::Char(c) => {
            let ch = if c == ' ' { '-' } else { c };
            app.new_entity_input.push(ch);
        }
        _ => {}
    }
}

pub fn handle_rename_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.submit_rename_selected_entity(),
        KeyCode::Esc => app.show_rename_popup = false,
        KeyCode::Backspace => {
            app.rename_input.pop();
        }
        KeyCode::Char(c) => {
            let ch = if c == ' ' { '-' } else { c };
            app.rename_input.push(ch);
        }
        _ => {}
    }
}

pub fn handle_delete_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let mut next = app.delete_selected_index;
            while next > 0 {
                next -= 1;
                if app.delete_options[next].2 {
                    app.delete_selected_index = next;
                    break;
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let mut next = app.delete_selected_index;
            while next + 1 < app.delete_options.len() {
                next += 1;
                if app.delete_options[next].2 {
                    app.delete_selected_index = next;
                    break;
                }
            }
        }
        KeyCode::Enter | KeyCode::Char('y') => app.confirm_delete_selected_entity(),
        KeyCode::Esc | KeyCode::Char('n') => app.show_delete_popup = false,
        _ => {}
    }
}
