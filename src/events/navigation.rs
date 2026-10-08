use crate::app::App;
use crate::ui::main_page::ActiveBlock;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_global_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,

        KeyCode::Tab | KeyCode::Char('h') => app.active_block = app.active_block.next(),
        KeyCode::BackTab | KeyCode::Char('l') => app.active_block = app.active_block.previous(),

        KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),

        KeyCode::Char(c @ '1'..='5') => {
            if let Some(digit) = c.to_digit(10)
                && let Some(block) = ActiveBlock::from_index((digit - 1) as usize)
            {
                app.active_block = block;
            }
        }

        KeyCode::Char('[') if app.active_block == ActiveBlock::Branches => {
            app.select_branch_tab(app.branch_tab.next());
        }
        KeyCode::Char(']') if app.active_block == ActiveBlock::Branches => {
            app.select_branch_tab(app.branch_tab.next());
        }

        KeyCode::Char('n') if app.active_block == ActiveBlock::Branches => {
            app.open_create_popup();
        }
        KeyCode::Enter if app.active_block == ActiveBlock::Branches => {
            app.checkout_selected_entity();
        }
        KeyCode::Char('d') if app.active_block == ActiveBlock::Branches => {
            app.open_delete_popup();
        }
        KeyCode::Char('r') if app.active_block == ActiveBlock::Branches => {
            app.open_rename_popup();
        }

        KeyCode::Char('?') => {
            app.show_about = false;
            app.show_help = !app.show_help;
        }
        KeyCode::Char('a') => {
            app.show_help = false;
            app.show_about = !app.show_about;
        }

        KeyCode::Char('p') if app.active_block == ActiveBlock::Branches => {
            app.pull_selected_branch();
        }
        KeyCode::Char('P') if app.active_block == ActiveBlock::Branches => {
            app.push_selected_branch();
        }

        _ => {}
    }
}
