use crate::git::{GitTarget, GitData};
use crate::ui::main_page::{ActiveBlock, BranchTab};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct App {
    pub active_block: ActiveBlock,
    pub branch_tab: BranchTab,
    pub selected_branch_index: usize,
    pub selected_file_index: usize,
    pub selected_commit_index: usize,

    pub git_data: GitData,
    pub should_quit: bool,
    pub tick_count: usize,

    pub show_help: bool,
    pub show_about: bool,
    pub show_create_popup: bool,
    pub new_entity_input: String,
    pub create_error_message: Option<String>,

    pub show_delete_popup: bool,
    pub delete_error_message: Option<String>,

    pub show_rename_popup: bool,
    pub rename_input: String,
    pub rename_error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_block: ActiveBlock::Files,
            branch_tab: BranchTab::Local,
            git_data: GitData::fetch_branches(),
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        if self.tick_count >= 8 {
            self.tick_count = 0;
            self.refresh_git();
        }
    }

    pub fn refresh_git(&mut self) {
        self.git_data = GitData::fetch_branches();
    }

    pub fn open_create_popup(&mut self) {
        self.show_create_popup = true;
        self.new_entity_input.clear();
        self.create_error_message = None;
    }

    pub fn close_create_popup(&mut self) {
        self.show_create_popup = false;
        self.new_entity_input.clear();
        self.create_error_message = None;
    }

    pub fn submit_create_entity(&mut self) {
        let target: GitTarget = self.branch_tab.into();

        match GitData::create_entity(target, &self.new_entity_input) {
            Ok(()) => {
                self.refresh_git();
                self.close_create_popup();
            }
            Err(err) => {
                self.create_error_message = Some(err);
            }
        }
    }

    pub fn get_selected_entity_name(&self) -> Option<String> {
        match self.branch_tab {
            BranchTab::Local => self
                .git_data
                .local_branches
                .get(self.selected_branch_index)
                .map(|b| b.name.clone()),
            BranchTab::Remote => self
                .git_data
                .remote_branches
                .get(self.selected_branch_index)
                .cloned(),
            BranchTab::Tags => self
                .git_data
                .tags
                .get(self.selected_branch_index)
                .cloned(),
        }
    }

    pub fn checkout_selected_entity(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();
            if let Err(err) = GitData::checkout_entity(target, &name) {
                eprintln!("Checkout error: {}", err);
            } else {
                self.refresh_git();
            }
        }
    }

    pub fn confirm_delete_selected_entity(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();
            match GitData::delete_entity(target, &name) {
                Ok(()) => {
                    self.show_delete_popup = false;
                    self.selected_branch_index = 0;
                    self.refresh_git();
                }
                Err(err) => {
                    self.delete_error_message = Some(err);
                }
            }
        }
    }

    pub fn submit_rename_selected_entity(&mut self) {
        if let Some(old_name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();
            match GitData::rename_entity(target, &old_name, &self.rename_input) {
                Ok(()) => {
                    self.show_rename_popup = false;
                    self.rename_input.clear();
                    self.refresh_git();
                }
                Err(err) => {
                    self.rename_error_message = Some(err);
                }
            }
        }
    }

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

        if self.show_create_popup {
            match key.code {
                KeyCode::Enter => self.submit_create_entity(),
                KeyCode::Esc => self.close_create_popup(),
                KeyCode::Backspace => {
                    self.new_entity_input.pop();
                }
                KeyCode::Char(c) => {
                    let ch = if c == ' ' { '-' } else { c };
                    self.new_entity_input.push(ch);
                }
                _ => {}
            }
            return;
        }

        if self.show_rename_popup {
            match key.code {
                KeyCode::Enter => self.submit_rename_selected_entity(),
                KeyCode::Esc => self.show_rename_popup = false,
                KeyCode::Backspace => {
                    self.rename_input.pop();
                }
                KeyCode::Char(c) => {
                    let ch = if c == ' ' { '-' } else { c };
                    self.rename_input.push(ch);
                }
                _ => {}
            }
            return;
        }

        if self.show_delete_popup {
            match key.code {
                KeyCode::Enter | KeyCode::Char('y') => self.confirm_delete_selected_entity(),
                KeyCode::Esc | KeyCode::Char('n') => self.show_delete_popup = false,
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,

            KeyCode::Tab | KeyCode::Char('h') => self.active_block = self.active_block.next(),
            KeyCode::BackTab | KeyCode::Char('l') => self.active_block = self.active_block.previous(),

            KeyCode::Down | KeyCode::Char('j') => self.move_selection_down(),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection_up(),

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

            KeyCode::Char('n') if self.active_block == ActiveBlock::Branches => {
                self.open_create_popup();
            }
            KeyCode::Enter if self.active_block == ActiveBlock::Branches => {
                self.checkout_selected_entity();
            }
            KeyCode::Char('d') if self.active_block == ActiveBlock::Branches => {
                self.show_delete_popup = true;
                self.delete_error_message = None;
            }
            KeyCode::Char('r') if self.active_block == ActiveBlock::Branches => {
                if let Some(name) = self.get_selected_entity_name() {
                    self.rename_input = name;
                    self.show_rename_popup = true;
                    self.rename_error_message = None;
                }
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


    fn move_selection_down(&mut self) {
        match self.active_block {
            ActiveBlock::Branches => {
                let count = self.get_branch_count();
                if count > 0 {
                    self.selected_branch_index = (self.selected_branch_index + 1) % count;
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
                    self.selected_branch_index = if self.selected_branch_index == 0 {
                        count - 1
                    } else {
                        self.selected_branch_index - 1
                    };
                }
            }
            ActiveBlock::Files => {
                self.selected_file_index = self.selected_file_index.saturating_sub(1);
            }
            ActiveBlock::CommitHistory => {
                self.selected_commit_index = self.selected_commit_index.saturating_sub(1);
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
}

impl From<BranchTab> for GitTarget {
    fn from(tab: BranchTab) -> Self {
        match tab {
            BranchTab::Local => GitTarget::LocalBranch,
            BranchTab::Remote => GitTarget::RemoteBranch,
            BranchTab::Tags => GitTarget::Tag,
        }
    }
}