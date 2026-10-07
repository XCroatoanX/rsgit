use crate::git::GitData;
use crate::ui::main_page::{ActiveBlock, BranchTab};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct App {
    pub active_block: ActiveBlock,
    pub branch_tab: BranchTab,
    pub should_quit: bool,
    pub git_data: GitData,
    pub selected_branch_index: usize,
    pub selected_file_index: usize,
    pub selected_commit_index: usize,
    pub show_help: bool,
    pub show_about: bool,
    pub tick_count: usize,
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

            KeyCode::Tab | KeyCode::Char('h') => {
                self.active_block = self.active_block.next();
            }
            KeyCode::BackTab | KeyCode::Char('l') => {
                self.active_block = self.active_block.previous();
            }

            KeyCode::Down | KeyCode::Char('j') => {
                self.move_selection_down();
            }

            KeyCode::Up | KeyCode::Char('k') => {
                self.move_selection_up();
            }

            KeyCode::Char(c @ '1'..='5') => {
                if let Some(digit) = c.to_digit(10)
                    && let Some(block) = ActiveBlock::from_index((digit - 1) as usize)
                {
                    self.active_block = block;
                }
            }

            KeyCode::Char('[') if self.active_block == ActiveBlock::Branches => {
                self.branch_tab = self.branch_tab.previous();
                self.selected_branch_index = 0;
            }
            KeyCode::Char(']') if self.active_block == ActiveBlock::Branches => {
                self.branch_tab = self.branch_tab.next();
                self.selected_branch_index = 0;
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

    pub fn new() -> Self {
        Self {
            active_block: ActiveBlock::Files,
            branch_tab: BranchTab::Local,
            should_quit: false,
            git_data: GitData::fetch_branches(),
            selected_branch_index: 0,
            selected_file_index: 0,
            selected_commit_index: 0,
            show_help: false,
            show_about: false,
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        if self.tick_count >= 8 {
            self.tick_count = 0;
            self.refresh_git();
        }
    }

    fn move_selection_down(&mut self) {
        match self.active_block {
            ActiveBlock::Branches => {
                let count = self.get_branch_count();
                if count > 0 {
                    self.selected_branch_index = (self.selected_branch_index + 1) % count;
                } else {
                    self.selected_branch_index = 0;
                }
            }
            ActiveBlock::Files => {}
            ActiveBlock::CommitHistory => {}
            _ => {}
        }
    }

    fn move_selection_up(&mut self) {
        match self.active_block {
            ActiveBlock::Branches => {
                let count = self.get_branch_count();
                if count > 0 {
                    if self.selected_branch_index == 0 {
                        self.selected_branch_index = count - 1;
                    } else {
                        self.selected_branch_index -= 1;
                    }
                } else {
                    self.selected_branch_index = 0;
                }
            }
            ActiveBlock::Files => {
                if self.selected_file_index > 0 {
                    self.selected_file_index -= 1;
                }
            }
            ActiveBlock::CommitHistory => {
                if self.selected_commit_index > 0 {
                    self.selected_commit_index -= 1;
                }
            }
            _ => {}
        }
    }

    fn get_branch_count(&self) -> usize {
        match self.branch_tab {
            BranchTab::Local => self.git_data.local_branches.len(),
            BranchTab::Remote => self.git_data.remote_branches.len(),
            BranchTab::Tags => self.git_data.tags.len(),
        }
    }

    pub fn refresh_git(&mut self) {
        self.git_data = GitData::fetch_branches();
    }
}
