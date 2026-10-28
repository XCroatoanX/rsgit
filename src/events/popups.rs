use crate::app::{App, delete_char, insert_char};
use crate::state::{AuthField, CommitField, Popup};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_create_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.submit_create_entity(),
        KeyCode::Esc => app.close_create_popup(),
        KeyCode::Left => {
            app.new_entity_cursor = app.new_entity_cursor.saturating_sub(1);
        }
        KeyCode::Right => {
            app.new_entity_cursor =
                (app.new_entity_cursor + 1).min(app.new_entity_input.chars().count());
        }
        KeyCode::Backspace => delete_char(&mut app.new_entity_input, &mut app.new_entity_cursor),
        KeyCode::Char(c) => {
            let ch = if c == ' ' { '-' } else { c };
            insert_char(&mut app.new_entity_input, &mut app.new_entity_cursor, ch);
        }
        _ => {}
    }
}

pub fn handle_rename_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.submit_rename_selected_entity(),
        KeyCode::Esc => app.popup = Popup::None,
        KeyCode::Left => {
            app.rename_cursor = app.rename_cursor.saturating_sub(1);
        }
        KeyCode::Right => {
            app.rename_cursor = (app.rename_cursor + 1).min(app.rename_input.chars().count());
        }
        KeyCode::Backspace => delete_char(&mut app.rename_input, &mut app.rename_cursor),
        KeyCode::Char(c) => {
            let ch = if c == ' ' { '-' } else { c };
            insert_char(&mut app.rename_input, &mut app.rename_cursor, ch);
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
        KeyCode::Esc | KeyCode::Char('n') => app.popup = Popup::None,
        _ => {}
    }
}

pub fn handle_merge_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.popup = Popup::None;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.merge_selected_index > 0 {
                app.merge_selected_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.merge_selected_index + 1 < app.merge_options.len() {
                app.merge_selected_index += 1;
            }
        }
        KeyCode::Enter => {
            app.confirm_merge_selected_entity();
        }
        _ => {}
    }
}

pub fn handle_rebase_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char('y') => app.confirm_rebase_selected_entity(),
        KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
            app.popup = Popup::None;
        }
        _ => {}
    }
}

pub fn handle_commit_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            if app.commit_field == CommitField::Title {
                app.submit_commit();
            } else {
                insert_char(
                    &mut app.commit_message,
                    &mut app.commit_message_cursor,
                    '\n',
                );
            }
        }
        KeyCode::Esc => app.close_commit_popup(),
        KeyCode::Tab => {
            app.commit_field = match app.commit_field {
                CommitField::Title => CommitField::Message,
                CommitField::Message => CommitField::Title,
            };
        }
        KeyCode::Left => match app.commit_field {
            CommitField::Title => {
                app.commit_title_cursor = app.commit_title_cursor.saturating_sub(1);
            }
            CommitField::Message => {
                app.commit_message_cursor = app.commit_message_cursor.saturating_sub(1);
            }
        },
        KeyCode::Right => match app.commit_field {
            CommitField::Title => {
                app.commit_title_cursor =
                    (app.commit_title_cursor + 1).min(app.commit_title.chars().count());
            }
            CommitField::Message => {
                app.commit_message_cursor =
                    (app.commit_message_cursor + 1).min(app.commit_message.chars().count());
            }
        },
        KeyCode::Backspace => match app.commit_field {
            CommitField::Title => {
                delete_char(&mut app.commit_title, &mut app.commit_title_cursor);
            }
            CommitField::Message => {
                delete_char(&mut app.commit_message, &mut app.commit_message_cursor);
            }
        },
        KeyCode::Char(c) => match app.commit_field {
            CommitField::Title => {
                insert_char(&mut app.commit_title, &mut app.commit_title_cursor, c);
            }
            CommitField::Message => {
                insert_char(&mut app.commit_message, &mut app.commit_message_cursor, c);
            }
        },
        _ => {}
    }
}

pub fn handle_auth_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => app.submit_auth(),
        KeyCode::Esc => app.close_auth_popup(),
        KeyCode::Tab => {
            app.auth_field = match app.auth_field {
                AuthField::Username => AuthField::Password,
                AuthField::Password => AuthField::Username,
            };
        }
        KeyCode::Left => match app.auth_field {
            AuthField::Username => {
                app.auth_username_cursor = app.auth_username_cursor.saturating_sub(1);
            }
            AuthField::Password => {
                app.auth_password_cursor = app.auth_password_cursor.saturating_sub(1);
            }
        },
        KeyCode::Right => match app.auth_field {
            AuthField::Username => {
                app.auth_username_cursor =
                    (app.auth_username_cursor + 1).min(app.auth_username.chars().count());
            }
            AuthField::Password => {
                app.auth_password_cursor =
                    (app.auth_password_cursor + 1).min(app.auth_password.chars().count());
            }
        },
        KeyCode::Backspace => match app.auth_field {
            AuthField::Username => {
                delete_char(&mut app.auth_username, &mut app.auth_username_cursor);
            }
            AuthField::Password => {
                delete_char(&mut app.auth_password, &mut app.auth_password_cursor);
            }
        },
        KeyCode::Char(c) => match app.auth_field {
            AuthField::Username => {
                insert_char(&mut app.auth_username, &mut app.auth_username_cursor, c);
            }
            AuthField::Password => {
                insert_char(&mut app.auth_password, &mut app.auth_password_cursor, c);
            }
        },
        _ => {}
    }
}
