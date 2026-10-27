use crate::git::{CommitLine, GitData, GitTarget};
use crate::state::{ActiveBlock, BranchTab, Popup};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

const LOCAL_REFRESH_INTERVAL_TICKS: usize = 8;
const REMOTE_FETCH_INTERVAL_TICKS: usize = 600;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AfterAction {
    #[default]
    None,
    SwitchToLocalBranches,
}

pub enum GitWorkerResult {
    Refresh(GitData),
    Fetch {
        result: Result<(), String>,
        git_data: GitData,
    },
    BranchHistory {
        target: String,
        history: Vec<CommitLine>,
    },
    CheckedOutHistory(Vec<CommitLine>),
    CommitDiff {
        commit: String,
        diff: Vec<String>,
    },
    Action {
        description: String,
        after: AfterAction,
        result: Result<(), String>,
    },
    Sync {
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
    pub selected_branch_history_index: usize,
    pub selected_log_index: usize,
    pub should_quit: bool,
    pub tick_count: usize,
    pub ticks_since_fetch: usize,

    pub popup: Popup,

    pub new_entity_input: String,
    pub create_error_message: Option<String>,

    pub delete_options: Vec<(DeleteScope, &'static str, bool)>,
    pub delete_selected_index: usize,
    pub delete_error_message: Option<String>,

    pub rename_input: String,
    pub rename_warning: Option<String>,
    pub rename_error_message: Option<String>,

    pub error_message: Option<String>,

    pub active_sync: Option<ActiveSync>,
    pub active_action: Option<String>,
    pub refresh_in_progress: bool,
    pub fetch_in_progress: bool,
    pub branch_history_in_progress: bool,
    pub branch_history_pending: bool,
    pub checked_out_history_in_progress: bool,
    pub checked_out_history_pending: bool,
    pub commit_diff_in_progress: bool,
    pub commit_diff_pending: bool,
    pub sync_tx: Option<Sender<GitWorkerResult>>,
    pub sync_rx: Option<Receiver<GitWorkerResult>>,
    pub merge_options: Vec<(MergeStrategy, &'static str, bool)>,
    pub merge_selected_index: usize,
    pub merge_source: String,
    pub merge_target: String,
    pub rebase_source: String,
    pub rebase_target: String,
    pub git_log: Vec<String>,
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
            ticks_since_fetch: REMOTE_FETCH_INTERVAL_TICKS,
            ..Default::default()
        };
        app.refresh_git();
        app
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        self.ticks_since_fetch += 1;

        self.drain_worker_messages();

        if self.tick_count >= LOCAL_REFRESH_INTERVAL_TICKS {
            self.tick_count = 0;
            if self.active_sync.is_none() && self.active_action.is_none() && !self.fetch_in_progress
            {
                self.refresh_git();
            }
        }

        if self.ticks_since_fetch >= REMOTE_FETCH_INTERVAL_TICKS
            && self.active_sync.is_none()
            && self.active_action.is_none()
            && !self.refresh_in_progress
            && !self.fetch_in_progress
        {
            self.ticks_since_fetch = 0;
            self.fetch_remotes();
        }
    }

    fn drain_worker_messages(&mut self) {
        let mut messages = Vec::new();
        if let Some(rx) = &self.sync_rx {
            while let Ok(msg) = rx.try_recv() {
                messages.push(msg);
            }
        }

        for msg in messages {
            self.handle_worker_message(msg);
        }
    }

    fn handle_worker_message(&mut self, msg: GitWorkerResult) {
        match msg {
            GitWorkerResult::Refresh(git_data) => {
                self.git_data = git_data;
                self.refresh_in_progress = false;
                self.clamp_branch_selection();
                self.request_branch_history();
                self.request_checked_out_history();
            }
            GitWorkerResult::Fetch { result, git_data } => {
                self.fetch_in_progress = false;
                if let Err(err) = result {
                    self.push_log(format!("Fetch failed: {}", err));
                }
                self.git_data = git_data;
                self.clamp_branch_selection();
                self.request_branch_history();
                self.request_checked_out_history();
            }
            GitWorkerResult::BranchHistory { target, history } => {
                self.branch_history_in_progress = false;
                if self.get_selected_entity_name().as_deref() == Some(target.as_str()) {
                    self.branch_history = history;
                    self.selected_branch_history_index = self
                        .selected_branch_history_index
                        .min(self.branch_history.len().saturating_sub(1));
                }

                if self.branch_history_pending {
                    self.branch_history_pending = false;
                    self.request_branch_history();
                }
            }
            GitWorkerResult::CheckedOutHistory(history) => {
                self.checked_out_history_in_progress = false;
                self.checked_out_history = history;
                self.selected_commit_index = self
                    .selected_commit_index
                    .min(self.checked_out_history.len().saturating_sub(1));

                if self.checked_out_history_pending {
                    self.checked_out_history_pending = false;
                    self.request_checked_out_history();
                } else {
                    self.request_commit_diff();
                }
            }
            GitWorkerResult::CommitDiff { commit, diff } => {
                self.commit_diff_in_progress = false;
                if self.current_commit_hash().as_deref() == Some(commit.as_str()) {
                    self.commit_diff = diff;
                    self.selected_diff_index = self
                        .selected_diff_index
                        .min(self.commit_diff.len().saturating_sub(1));
                }

                if self.commit_diff_pending {
                    self.commit_diff_pending = false;
                    self.request_commit_diff();
                }
            }
            GitWorkerResult::Action {
                description,
                after,
                result,
            } => {
                self.active_action = None;
                match result {
                    Ok(()) => {
                        self.push_log(format!("Success: {}", description));
                        if after == AfterAction::SwitchToLocalBranches {
                            self.branch_tab = BranchTab::Local;
                            self.selected_branch_index = 0;
                        }
                        self.refresh_git();
                    }
                    Err(err) => {
                        let msg = format!("Failed ({}): {}", description, err);
                        self.push_log(&msg);
                        self.error_message = Some(msg);
                    }
                }
            }
            GitWorkerResult::Sync {
                branch,
                action,
                result,
            } => {
                self.active_sync = None;
                if let Err(err) = result {
                    let msg = format!("{:?} failed for '{}': {}", action, branch, err);
                    self.push_log(&msg);
                    self.error_message = Some(msg);
                } else {
                    self.push_log(format!("{:?} succeeded for '{}'", action, branch));
                }
                self.refresh_git();
            }
        }
    }

    pub fn run_git_action<F>(&mut self, action_desc: impl Into<String>, f: F)
    where
        F: FnOnce() -> Result<(), String> + Send + 'static,
    {
        self.run_git_action_after(action_desc, AfterAction::None, f);
    }

    pub fn run_git_action_after<F>(
        &mut self,
        action_desc: impl Into<String>,
        after: AfterAction,
        f: F,
    ) where
        F: FnOnce() -> Result<(), String> + Send + 'static,
    {
        if self.active_action.is_some() || self.active_sync.is_some() {
            self.push_log("Another git action is already running.");
            return;
        }

        let desc = action_desc.into();
        self.push_log(format!("Running: {}", desc));
        self.active_action = Some(desc.clone());

        let Some(tx) = self.sync_tx.clone() else {
            self.active_action = None;
            return;
        };

        thread::spawn(move || {
            let result = f();
            let _ = tx.send(GitWorkerResult::Action {
                description: desc,
                after,
                result,
            });
        });
    }

    pub fn pull_selected_branch(&mut self) {
        if self.active_sync.is_some() || self.active_action.is_some() {
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.push_log(format!("Starting pull for '{}'", name));
            self.active_sync = Some(ActiveSync {
                branch_name: name.clone(),
                action: SyncAction::Pull,
            });

            if let Some(tx) = self.sync_tx.clone() {
                let branch = name.clone();
                thread::spawn(move || {
                    let res = GitData::pull_branch(&branch);
                    let _ = tx.send(GitWorkerResult::Sync {
                        branch,
                        action: SyncAction::Pull,
                        result: res,
                    });
                });
            }
        }
    }

    pub fn push_selected_branch(&mut self) {
        if self.active_sync.is_some() || self.active_action.is_some() {
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.push_log(format!("Starting push for '{}'", name));
            self.active_sync = Some(ActiveSync {
                branch_name: name.clone(),
                action: SyncAction::Push,
            });

            if let Some(tx) = self.sync_tx.clone() {
                let branch = name.clone();
                thread::spawn(move || {
                    let res = GitData::push_branch(&branch);
                    let _ = tx.send(GitWorkerResult::Sync {
                        branch,
                        action: SyncAction::Push,
                        result: res,
                    });
                });
            }
        }
    }

    pub fn refresh_git(&mut self) {
        if self.refresh_in_progress || self.fetch_in_progress {
            return;
        }

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.refresh_in_progress = true;
        thread::spawn(move || {
            let git_data = GitData::load_local();
            let _ = tx.send(GitWorkerResult::Refresh(git_data));
        });
    }

    pub fn fetch_remotes(&mut self) {
        if self.fetch_in_progress || self.refresh_in_progress {
            return;
        }

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.fetch_in_progress = true;
        thread::spawn(move || {
            let result = GitData::fetch_remotes();
            let git_data = GitData::load_local();
            let _ = tx.send(GitWorkerResult::Fetch { result, git_data });
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

    pub fn request_branch_history(&mut self) {
        if self.branch_history_in_progress {
            self.branch_history_pending = true;
            return;
        }

        let Some(target) = self.get_selected_entity_name() else {
            self.branch_history.clear();
            self.selected_branch_history_index = 0;
            return;
        };

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.branch_history_in_progress = true;
        thread::spawn(move || {
            let history = GitData::fetch_history(&target);
            let _ = tx.send(GitWorkerResult::BranchHistory { target, history });
        });
    }

    pub fn request_checked_out_history(&mut self) {
        if self.checked_out_history_in_progress {
            self.checked_out_history_pending = true;
            return;
        }

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.checked_out_history_in_progress = true;
        thread::spawn(move || {
            let history = GitData::fetch_checked_out_history();
            let _ = tx.send(GitWorkerResult::CheckedOutHistory(history));
        });
    }

    pub fn request_commit_diff(&mut self) {
        if self.commit_diff_in_progress {
            self.commit_diff_pending = true;
            return;
        }

        let Some(commit) = self.current_commit_hash() else {
            self.commit_diff.clear();
            self.selected_diff_index = 0;
            return;
        };

        let Some(tx) = self.sync_tx.clone() else {
            return;
        };

        self.commit_diff_in_progress = true;
        thread::spawn(move || {
            let diff = GitData::fetch_commit_diff(&commit);
            let _ = tx.send(GitWorkerResult::CommitDiff { commit, diff });
        });
    }

    fn current_commit_hash(&self) -> Option<String> {
        self.checked_out_history
            .get(self.selected_commit_index)
            .and_then(|commit| commit.hash.clone())
    }

    pub fn select_branch_tab(&mut self, branch_tab: BranchTab) {
        self.branch_tab = branch_tab;
        self.selected_branch_index = 0;
        self.request_branch_history();
    }

    pub fn checkout_selected_entity(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();
            let desc = format!("checkout {:?} '{}'", target, name);
            let after = if self.branch_tab == BranchTab::Remote {
                AfterAction::SwitchToLocalBranches
            } else {
                AfterAction::None
            };

            self.run_git_action_after(desc, after, move || GitData::checkout_entity(target, &name));
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.active_block {
            ActiveBlock::Branches => {
                let count = self.get_branch_count();
                if count > 0 {
                    self.selected_branch_index = (self.selected_branch_index + 1) % count;
                    self.request_branch_history();
                }
            }
            ActiveBlock::Files => {}
            ActiveBlock::CommitHistory => {
                if !self.checked_out_history.is_empty() {
                    self.selected_commit_index =
                        (self.selected_commit_index + 1) % self.checked_out_history.len();
                    self.selected_diff_index = 0;
                    self.request_commit_diff();
                }
            }
            ActiveBlock::Diff if !self.commit_diff.is_empty() => {
                self.selected_diff_index = (self.selected_diff_index + 1) % self.commit_diff.len();
            }
            ActiveBlock::Logs if !self.git_log.is_empty() => {
                self.selected_log_index = (self.selected_log_index + 1) % self.git_log.len();
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
                    self.request_branch_history();
                }
            }
            ActiveBlock::Files => {
                self.selected_file_index = self.selected_file_index.saturating_sub(1);
            }
            ActiveBlock::CommitHistory => {
                if self.selected_commit_index > 0 {
                    self.selected_commit_index -= 1;
                    self.selected_diff_index = 0;
                    self.request_commit_diff();
                }
            }
            ActiveBlock::Diff if !self.commit_diff.is_empty() => {
                self.selected_diff_index = self.selected_diff_index.saturating_sub(1);
            }
            ActiveBlock::Logs if self.selected_log_index > 0 => {
                self.selected_log_index -= 1;
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

    fn clamp_branch_selection(&mut self) {
        self.selected_branch_index = self
            .selected_branch_index
            .min(self.get_branch_count().saturating_sub(1));
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
        self.popup = Popup::Rebase;
    }

    pub fn confirm_rebase_selected_entity(&mut self) {
        self.popup = Popup::None;

        let target = std::mem::take(&mut self.rebase_target);
        self.rebase_source.clear();

        if target.is_empty() {
            return;
        }

        self.run_git_action(format!("rebase branch onto '{}'", target), move || {
            GitData::rebase_branch(&target)
        });
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
            GitData::can_fast_forward_into(&target)
        } else {
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
        self.popup = Popup::Merge;
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

        self.popup = Popup::None;

        let source = std::mem::take(&mut self.merge_source);
        let target = std::mem::take(&mut self.merge_target);
        let is_local = self.branch_tab == BranchTab::Local;

        let desc = format!("merge '{}' into '{}' with {:?}", source, target, strategy);

        self.run_git_action(desc, move || {
            if is_local {
                GitData::merge_into(&target, &source, strategy)
            } else {
                GitData::merge_with_strategy(&target, strategy)
            }
        });
    }

    pub fn push_log(&mut self, message: impl Into<String>) {
        let follow_tail = self.selected_log_index + 1 >= self.git_log.len();

        self.git_log.push(message.into());
        if self.git_log.len() > 500 {
            self.git_log.remove(0);
            self.selected_log_index = self.selected_log_index.saturating_sub(1);
        }

        if follow_tail {
            self.selected_log_index = self.git_log.len().saturating_sub(1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::BranchInfo;

    fn app_with_branch(name: &str) -> App {
        let mut app = App::default();
        app.git_data.local_branches = vec![BranchInfo {
            name: name.into(),
            ahead: 0,
            behind: 0,
            is_head: true,
        }];
        app
    }

    fn commit(hash: &str) -> CommitLine {
        CommitLine {
            graph: "*".into(),
            hash: Some(hash.into()),
            author: Some("me".into()),
            subject: Some("a commit".into()),
        }
    }

    #[test]
    fn branch_history_refresh_does_not_move_the_commit_diff_selection() {
        let mut app = app_with_branch("main");
        app.commit_diff = (0..50).map(|i| format!("line {i}")).collect();
        app.selected_diff_index = 30;

        app.handle_worker_message(GitWorkerResult::BranchHistory {
            target: "main".into(),
            history: vec![commit("abc1234")],
        });

        assert_eq!(app.selected_diff_index, 30);
        assert_eq!(app.selected_branch_history_index, 0);
        assert_eq!(app.branch_history.len(), 1);
    }

    #[test]
    fn commit_diff_refresh_clamps_only_the_diff_selection() {
        let mut app = app_with_branch("main");
        app.commit_diff = (0..50).map(|i| format!("line {i}")).collect();
        app.selected_diff_index = 30;
        app.selected_branch_history_index = 2;
        app.checked_out_history = vec![commit("abc1234")];

        app.handle_worker_message(GitWorkerResult::CommitDiff {
            commit: "abc1234".into(),
            diff: (0..10).map(|i| format!("line {i}")).collect(),
        });

        assert_eq!(app.selected_diff_index, 9);
        assert_eq!(app.selected_branch_history_index, 2);
    }
}
