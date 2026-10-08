use crate::git::{GitData, GitTarget};
use crate::ui::main_page::{ActiveBlock, BranchTab};
use crossterm::event::{KeyCode, KeyEvent};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeleteScope {
    LocalOnly,
    RemoteOnly,
    Both,
    Cancel,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyncAction {
    Pull,
    Push,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveSync {
    pub branch_name: String,
    pub action: SyncAction,
}

pub enum GitWorkerResult {
    SyncCompleted {
        branch: String,
        action: SyncAction,
        result: Result<(), String>,
    },
}

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
    pub delete_options: Vec<(DeleteScope, &'static str, bool)>,
    pub delete_selected_index: usize,
    pub delete_error_message: Option<String>,

    pub show_rename_popup: bool,
    pub rename_input: String,
    pub rename_warning: Option<String>,
    pub rename_error_message: Option<String>,

    pub error_message: Option<String>,

    pub active_sync: Option<ActiveSync>,
    pub sync_tx: Option<Sender<GitWorkerResult>>,
    pub sync_rx: Option<Receiver<GitWorkerResult>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            active_block: ActiveBlock::Files,
            branch_tab: BranchTab::Local,
            git_data: GitData::fetch_branches(),
            sync_tx: Some(tx),
            sync_rx: Some(rx),
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        let mut messages = Vec::new();
        if let Some(rx) = &self.sync_rx {
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }

        for msg in messages {
            match msg {
                GitWorkerResult::SyncCompleted {
                    branch,
                    action,
                    result,
                } => {
                    self.active_sync = None;
                    if let Err(err) = result {
                        self.error_message =
                            Some(format!("{:?} failed for '{}': {}", action, branch, err));
                    }
                    self.refresh_git();
                }
            }
        }

        if self.tick_count >= 8 {
            self.tick_count = 0;
            if self.active_sync.is_none() {
                self.refresh_git();
            }
        }
    }

    pub fn pull_selected_branch(&mut self) {
        if self.active_sync.is_some() {
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.active_sync = Some(ActiveSync {
                branch_name: name.clone(),
                action: SyncAction::Pull,
            });

            if let Some(tx) = self.sync_tx.clone() {
                let branch = name.clone();
                thread::spawn(move || {
                    let res = GitData::pull_branch(&branch);
                    let _ = tx.send(GitWorkerResult::SyncCompleted {
                        branch,
                        action: SyncAction::Pull,
                        result: res,
                    });
                });
            }
        }
    }

    pub fn push_selected_branch(&mut self) {
        if self.active_sync.is_some() {
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.active_sync = Some(ActiveSync {
                branch_name: name.clone(),
                action: SyncAction::Push,
            });

            if let Some(tx) = self.sync_tx.clone() {
                let branch = name.clone();
                thread::spawn(move || {
                    let res = GitData::push_branch(&branch);
                    let _ = tx.send(GitWorkerResult::SyncCompleted {
                        branch,
                        action: SyncAction::Push,
                        result: res,
                    });
                });
            }
        }
    }

    pub fn refresh_git(&mut self) {
        let _ = std::process::Command::new("git")
            .args(["fetch", "--prune"])
            .output();

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

    pub fn open_delete_popup(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let has_local = self.git_data.local_branches.iter().any(|b| b.name == name);
            let has_remote = self.git_data.remote_branches.iter().any(|b| {
                let branch_name = b.split_once('/').map(|(_, rest)| rest).unwrap_or(b);
                branch_name == name || b == &name
            });

            self.delete_options = match self.branch_tab {
                BranchTab::Local => vec![
                    (
                        DeleteScope::LocalOnly,
                        "Delete Local Branch Only",
                        has_local,
                    ),
                    (
                        DeleteScope::Both,
                        "Delete Both (Local & Remote)",
                        has_local && has_remote,
                    ),
                    (
                        DeleteScope::RemoteOnly,
                        "Delete Remote Branch Only",
                        has_remote,
                    ),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
                BranchTab::Remote => vec![
                    (
                        DeleteScope::RemoteOnly,
                        "Delete Remote Branch Only",
                        has_remote,
                    ),
                    (
                        DeleteScope::Both,
                        "Delete Both (Local & Remote)",
                        has_local && has_remote,
                    ),
                    (
                        DeleteScope::LocalOnly,
                        "Delete Local Branch Only",
                        has_local,
                    ),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
                BranchTab::Tags => vec![
                    (DeleteScope::LocalOnly, "Delete Local Tag", true),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
            };

            self.delete_selected_index = self
                .delete_options
                .iter()
                .position(|(_, _, enabled)| *enabled)
                .unwrap_or(3);

            self.delete_error_message = None;
            self.show_delete_popup = true;
        }
    }

    pub fn confirm_delete_selected_entity(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            if self.delete_options.is_empty() {
                return;
            }

            let (scope, _, enabled) = self.delete_options[self.delete_selected_index];
            if !enabled {
                return;
            }

            self.show_delete_popup = false;

            let result = match scope {
                DeleteScope::LocalOnly => GitData::delete_entity(GitTarget::LocalBranch, &name),
                DeleteScope::RemoteOnly => GitData::delete_entity(GitTarget::RemoteBranch, &name),
                DeleteScope::Both => {
                    let local_res = GitData::delete_entity(GitTarget::LocalBranch, &name);
                    let remote_res = GitData::delete_entity(GitTarget::RemoteBranch, &name);

                    local_res.and(remote_res)
                }
                DeleteScope::Cancel => Ok(()),
            };

            if let Err(err) = result {
                self.error_message = Some(err);
            } else {
                self.selected_branch_index = 0;
                self.refresh_git();
            }
        }
    }

    pub fn open_rename_popup(&mut self) {
        if self.branch_tab == BranchTab::Remote {
            self.error_message = Some(
                "Renaming remote branches directly is not supported by Git. Create a new remote branch and delete the old one instead.".into(),
            );
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.rename_input = name.clone();

            let has_remote = self.git_data.remote_branches.iter().any(|b| {
                let branch_name = b.split_once('/').map(|(_, rest)| rest).unwrap_or(b);
                branch_name == name || b == &name
            });

            if has_remote {
                self.rename_warning = Some(
                    "Renaming only affects your local branch. Remote branch will not be changed."
                        .into(),
                );
            } else {
                self.rename_warning = None;
            }

            self.rename_error_message = None;
            self.show_rename_popup = true;
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
            BranchTab::Tags => self.git_data.tags.get(self.selected_branch_index).cloned(),
        }
    }

    pub fn checkout_selected_entity(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();

            if let Err(err) = GitData::checkout_entity(target, &name) {
                self.error_message = Some(err);
            } else {
                if self.branch_tab == BranchTab::Remote {
                    self.branch_tab = BranchTab::Local;
                    self.selected_branch_index = 0;
                }

                self.refresh_git();
            }
        }
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if self.error_message.is_some() {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q')) {
                self.error_message = None;
            }
            return;
        }

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
                KeyCode::Up | KeyCode::Char('k') => {
                    let mut next = self.delete_selected_index;
                    while next > 0 {
                        next -= 1;
                        if self.delete_options[next].2 {
                            self.delete_selected_index = next;
                            break;
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let mut next = self.delete_selected_index;
                    while next + 1 < self.delete_options.len() {
                        next += 1;
                        if self.delete_options[next].2 {
                            self.delete_selected_index = next;
                            break;
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Char('y') => self.confirm_delete_selected_entity(),
                KeyCode::Esc | KeyCode::Char('n') => self.show_delete_popup = false,
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.should_quit = true,

            KeyCode::Tab | KeyCode::Char('h') => self.active_block = self.active_block.next(),
            KeyCode::BackTab | KeyCode::Char('l') => {
                self.active_block = self.active_block.previous()
            }

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
                self.open_delete_popup();
            }
            KeyCode::Char('r') if self.active_block == ActiveBlock::Branches => {
                self.open_rename_popup();
            }

            KeyCode::Char('?') => {
                self.show_about = false;
                self.show_help = !self.show_help;
            }
            KeyCode::Char('a') => {
                self.show_help = false;
                self.show_about = !self.show_about;
            }

            KeyCode::Char('p') if self.active_block == ActiveBlock::Branches => {
                self.pull_selected_branch();
            }
            KeyCode::Char('P') if self.active_block == ActiveBlock::Branches => {
                self.push_selected_branch();
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
