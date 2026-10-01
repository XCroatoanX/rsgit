use crate::ui::main_page::{ActiveBlock, BranchTab};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct App {
    pub show_help: bool,
    pub show_about: bool,
    pub active_block: ActiveBlock,
    pub branch_tab: BranchTab,
    pub should_quit: bool,
}

impl App {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if self.show_help || self.show_about {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.show_help = false;
                    self.show_about = false;
                }
                KeyCode::Char('?') => {
                    self.show_about = false;
                    self.show_help = !self.show_help;
                }
                KeyCode::Char('a') => {
                    self.show_help = false;
                    self.show_about = !self.show_about;
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,

            // Cycle pane focus using Enum methods
            KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
                self.active_block = self.active_block.next();
            }
            KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
                self.active_block = self.active_block.previous();
            }

            KeyCode::Char(c @ '1'..='5') => {
                if let Some(digit) = c.to_digit(10) {
                    if let Some(block) = ActiveBlock::from_index((digit - 1) as usize) {
                        self.active_block = block;
                    }
                }
            }

            KeyCode::Char('[') if self.active_block == ActiveBlock::Branches => {
                self.branch_tab = self.branch_tab.previous();
            }
            KeyCode::Char(']') if self.active_block == ActiveBlock::Branches => {
                self.branch_tab = self.branch_tab.next();
            }

            KeyCode::Char('?') => {
                self.show_about = false;
                self.show_help = !self.show_help;
            }
            KeyCode::Char('a') => {
                self.show_help = false;
                self.show_about = !self.show_about;
            }
            _ => {}
        }
    }
}
