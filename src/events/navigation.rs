use crate::app::App;
use crate::state::{ActiveBlock, Popup};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_global_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,

        KeyCode::Tab | KeyCode::Char('h') => {
            app.diff_context = app.active_block.into();
            app.active_block = app.active_block.next();
        }
        KeyCode::BackTab | KeyCode::Char('l') => {
            app.diff_context = app.active_block.into();
            app.active_block = app.active_block.previous();
        }

        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),

        KeyCode::Char(c @ '1'..='5') => {
            if let Some(digit) = c.to_digit(10)
                && let Some(block) = ActiveBlock::from_index((digit - 1) as usize)
            {
                app.diff_context = app.active_block.into();
                app.active_block = block;
            }
        }

        KeyCode::Char('[') if app.active_block == ActiveBlock::Branches => {
            app.select_branch_tab(app.branch_tab.previous());
        }
        KeyCode::Char(']') if app.active_block == ActiveBlock::Branches => {
            app.select_branch_tab(app.branch_tab.next());
        }

        KeyCode::Enter if app.active_block == ActiveBlock::Files => {
            app.toggle_selected_folder();
        }
        KeyCode::Char(' ') if app.active_block == ActiveBlock::Files => {
            app.toggle_stage_selected_file();
        }
        KeyCode::Char('c') if app.active_block == ActiveBlock::Files => {
            app.open_commit_popup();
        }
        KeyCode::Char(' ') if app.active_block == ActiveBlock::Branches => {
            app.checkout_selected_entity();
        }
        KeyCode::Char('n') if app.active_block == ActiveBlock::Branches => {
            app.open_create_popup();
        }
        KeyCode::Char('d') if app.active_block == ActiveBlock::Branches => {
            app.open_delete_popup();
        }
        KeyCode::Char('r') if app.active_block == ActiveBlock::Branches => {
            app.open_rename_popup();
        }

        KeyCode::Char('a') if app.active_block == ActiveBlock::Files => {
            app.toggle_stage_all_files();
        }

        KeyCode::Char('?') => {
            app.popup = if app.popup == Popup::Help {
                Popup::None
            } else {
                Popup::Help
            };
        }
        KeyCode::Char('x') => {
            app.popup = if app.popup == Popup::About {
                Popup::None
            } else {
                Popup::About
            };
        }

        KeyCode::Char('p') if app.active_block == ActiveBlock::Branches => {
            app.pull_selected_branch();
        }
        KeyCode::Char('P') if app.active_block == ActiveBlock::Branches => {
            app.push_selected_branch();
        }

        KeyCode::Char('M') if app.active_block == ActiveBlock::Branches => {
            app.open_merge_popup();
        }
        KeyCode::Char('R') if app.active_block == ActiveBlock::Branches => {
            app.open_rebase_popup();
        }

        _ => {}
    }
}
