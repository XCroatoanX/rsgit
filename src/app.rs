use crate::git::{CommitLine, GitData, GitTarget};
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
pub enum MergeStrategy {
    FastForward,
    MergeCommit,
    SquashUncommitted,
    SquashCommit,
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
    pub branch_history: Vec<CommitLine>,
    pub checked_out_history: Vec<CommitLine>,
    pub commit_diff: Vec<String>,
    pub selected_diff_index: usize,
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
    pub show_merge_popup: bool,
    pub merge_options: Vec<(MergeStrategy, &'static str, bool)>,
    pub merge_selected_index: usize,
    pub merge_source: String,
    pub merge_target: String,
    pub show_rebase_popup: bool,
    pub rebase_source: String,
    pub rebase_target: String,
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
                    self.refresh_branch_history();
                    self.refresh_checked_out_history();
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

    pub fn refresh_branch_history(&mut self) {
        self.branch_history = self
            .get_selected_entity_name()
            .map(|name| GitData::fetch_history(&name))
            .unwrap_or_default();
        self.selected_diff_index = self
            .selected_diff_index
            .min(self.branch_history.len().saturating_sub(1));
    }

    pub fn refresh_checked_out_history(&mut self) {
        self.checked_out_history = GitData::fetch_checked_out_history();
        self.selected_commit_index = self
            .selected_commit_index
            .min(self.checked_out_history.len().saturating_sub(1));
        self.refresh_commit_diff();
    }

    pub fn refresh_commit_diff(&mut self) {
        self.commit_diff = self
            .checked_out_history
            .get(self.selected_commit_index)
            .and_then(|commit| commit.hash.as_deref())
            .map(GitData::fetch_commit_diff)
            .unwrap_or_default();
        self.selected_diff_index = self
            .selected_diff_index
            .min(self.commit_diff.len().saturating_sub(1));
    }

    pub fn select_branch_tab(&mut self, branch_tab: BranchTab) {
        self.branch_tab = branch_tab;
        self.selected_branch_index = 0;
        self.refresh_branch_history();
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
                    self.refresh_branch_history();
                }
            }
            ActiveBlock::Files => {}
            ActiveBlock::CommitHistory => {
                if !self.checked_out_history.is_empty() {
                    self.selected_commit_index =
                        (self.selected_commit_index + 1) % self.checked_out_history.len();
                    self.selected_diff_index = 0;
                    self.refresh_commit_diff();
                }
            }
            ActiveBlock::Diff if !self.commit_diff.is_empty() => {
                self.selected_diff_index = (self.selected_diff_index + 1) % self.commit_diff.len();
            }
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
                    self.refresh_branch_history();
                }
            }
            ActiveBlock::Files => {
                self.selected_file_index = self.selected_file_index.saturating_sub(1);
            }
            ActiveBlock::CommitHistory => {
                if self.selected_commit_index > 0 {
                    self.selected_commit_index -= 1;
                    self.selected_diff_index = 0;
                    self.refresh_commit_diff();
                }
            }
            ActiveBlock::Diff if !self.commit_diff.is_empty() => {
                self.selected_diff_index = self.selected_diff_index.saturating_sub(1);
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

    pub fn open_rebase_popup(&mut self) {
        let Some(target) = self.get_selected_entity_name() else {
            return;
        };

        let current = match GitData::current_branch() {
            Ok(current) => current,
            Err(err) => {
                self.error_message = Some(err);
                return;
            }
        };

        if current == target {
            self.error_message = Some(format!("'{}' is already checked out.", target));
            return;
        }

        if GitData::is_ancestor(&target, &current) {
            self.error_message = Some(format!(
                "Nothing to rebase: '{}' is already based on '{}'.",
                current, target
            ));
            return;
        }

        self.rebase_source = current;
        self.rebase_target = target;
        self.show_rebase_popup = true;
    }

    pub fn confirm_rebase_selected_entity(&mut self) {
        self.show_rebase_popup = false;

        let target = std::mem::take(&mut self.rebase_target);
        self.rebase_source.clear();

        if target.is_empty() {
            return;
        }

        if let Err(err) = GitData::rebase_branch(&target) {
            self.error_message = Some(format!("Rebase failed: {}", err));
        } else {
            self.refresh_git();
        }
    }

    pub fn open_merge_popup(&mut self) {
        let Some(target) = self.get_selected_entity_name() else {
            return;
        };

        let source = match GitData::current_branch() {
            Ok(source) => source,
            Err(err) => {
                self.error_message = Some(err);
                return;
            }
        };

        let ff_available = if self.branch_tab == BranchTab::Local {
            if source == target {
                self.error_message = Some(format!("'{}' is already checked out.", target));
                return;
            }
            if GitData::is_ancestor(&source, &target) {
                self.error_message = Some(format!(
                    "Nothing to merge: '{}' already contains '{}'.",
                    target, source
                ));
                return;
            }
            // Local branches are merge destinations: HEAD gets merged into them.
            GitData::can_fast_forward_into(&target)
        } else {
            // A remote/tag cannot receive a merge, so it is merged into HEAD.
            GitData::can_fast_forward_from(&target)
        };

        self.merge_options = vec![
            (
                MergeStrategy::FastForward,
                "Fast-forward only (--ff-only)",
                ff_available,
            ),
            (
                MergeStrategy::MergeCommit,
                "Create merge commit (--no-ff)",
                true,
            ),
            (
                MergeStrategy::SquashUncommitted,
                "Squash merge (Uncommitted)",
                true,
            ),
            (MergeStrategy::SquashCommit, "Squash merge & Commit", true),
            (MergeStrategy::Cancel, "Cancel", true),
        ];

        self.merge_selected_index = self
            .merge_options
            .iter()
            .position(|(_, _, enabled)| *enabled)
            .unwrap_or(4);

        self.merge_source = source;
        self.merge_target = target;
        self.show_merge_popup = true;
    }

    pub fn merge_popup_title(&self) -> String {
        if self.branch_tab == BranchTab::Local {
            format!("Merge '{}' into '{}'", self.merge_source, self.merge_target)
        } else {
            format!("Merge '{}' into '{}'", self.merge_target, self.merge_source)
        }
    }

    pub fn confirm_merge_selected_entity(&mut self) {
        if self.merge_options.is_empty() {
            return;
        }

        let (strategy, _, enabled) = self.merge_options[self.merge_selected_index];
        if !enabled {
            return;
        }

        self.show_merge_popup = false;

        let source = std::mem::take(&mut self.merge_source);
        let target = std::mem::take(&mut self.merge_target);

        let result = if self.branch_tab == BranchTab::Local {
            GitData::merge_into(&target, &source, strategy)
        } else {
            GitData::merge_with_strategy(&target, strategy)
        };

        if let Err(err) = result {
            self.error_message = Some(format!("Merge failed: {}", err));
        } else {
            self.refresh_git();
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
