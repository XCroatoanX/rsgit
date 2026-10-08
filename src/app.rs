use crate::git::{GitData, GitTarget};
use crate::ui::main_page::{ActiveBlock, BranchTab};
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
    RefreshCompleted(GitData),
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
    pub refresh_in_progress: bool,
    pub sync_tx: Option<Sender<GitWorkerResult>>,
    pub sync_rx: Option<Receiver<GitWorkerResult>>,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let mut app = Self {
            active_block: ActiveBlock::Files,
            branch_tab: BranchTab::Local,
            git_data: GitData::default(),
            sync_tx: Some(tx),
            sync_rx: Some(rx),
            ..Default::default()
        };
        app.refresh_git();
        app
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
                GitWorkerResult::RefreshCompleted(git_data) => {
                    self.git_data = git_data;
                    self.refresh_in_progress = false;
                }
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
        if self.refresh_in_progress {
            return;
        }

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.refresh_in_progress = true;
        thread::spawn(move || {
            let git_data = GitData::fetch_branches();
            let _ = tx.send(GitWorkerResult::RefreshCompleted(git_data));
        });
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

    pub fn move_selection_down(&mut self) {
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

    pub fn move_selection_up(&mut self) {
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

    pub fn get_branch_count(&self) -> usize {
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
