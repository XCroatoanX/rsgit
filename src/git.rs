pub mod branches;
mod commands;
mod commits;
mod sync;
mod types;

use crate::app::MergeStrategy;

pub use types::{BranchInfo, CommitLine, GitTarget};

#[derive(Debug, Clone, Default)]
pub struct GitData {
    pub local_branches: Vec<BranchInfo>,
    pub remote_branches: Vec<String>,
    pub tags: Vec<String>,
}

impl GitData {
    pub fn fetch_branches() -> Self {
        let _ = sync::fetch_remotes();
        Self {
            local_branches: branches::fetch_local_branches(),
            remote_branches: branches::fetch_remote_branches(),
            tags: branches::fetch_tags(),
        }
    }

    pub fn fetch_history(target: &str) -> Vec<CommitLine> {
        commits::fetch_history(target)
    }

    pub fn fetch_checked_out_history() -> Vec<CommitLine> {
        commits::fetch_checked_out_history()
    }

    pub fn fetch_commit_diff(commit: &str) -> Vec<String> {
        commits::fetch_commit_diff(commit)
    }

    pub fn create_entity(target: GitTarget, name: &str) -> Result<(), String> {
        branches::create_entity(target, name)
    }

    pub fn checkout_entity(target: GitTarget, name: &str) -> Result<(), String> {
        branches::checkout_entity(target, name)
    }

    pub fn delete_entity(target: GitTarget, name: &str) -> Result<(), String> {
        branches::delete_entity(target, name)
    }

    pub fn rename_entity(target: GitTarget, old_name: &str, new_name: &str) -> Result<(), String> {
        branches::rename_entity(target, old_name, new_name)
    }

    pub fn pull_branch(branch: &str) -> Result<(), String> {
        sync::pull_branch(branch)
    }

    pub fn push_branch(branch: &str) -> Result<(), String> {
        sync::push_branch(branch)
    }

    pub fn rebase_branch(branch: &str) -> Result<(), String> {
        branches::rebase_branch(branch)
    }

    pub fn current_branch() -> Result<String, String> {
        branches::current_branch()
    }

    pub fn can_fast_forward_from(branch: &str) -> bool {
        branches::can_fast_forward_from(branch)
    }

    pub fn can_fast_forward_into(branch: &str) -> bool {
        branches::can_fast_forward_into(branch)
    }

    pub fn is_ancestor(ancestor: &str, descendant: &str) -> bool {
        branches::is_ancestor(ancestor, descendant)
    }

    pub fn merge_into(target: &str, source: &str, strategy: MergeStrategy) -> Result<(), String> {
        branches::merge_into(target, source, strategy)
    }

    pub fn merge_with_strategy(branch: &str, strategy: MergeStrategy) -> Result<(), String> {
        branches::merge_with_strategy(branch, strategy)
    }
}
