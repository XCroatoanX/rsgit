pub mod branches;
mod commands;
mod commits;
mod sync;
mod types;

use crate::app::MergeStrategy;
use std::process::Command;
pub use types::{BranchInfo, CommitLine, GitTarget};

#[derive(Debug, Clone, Default)]
pub struct GitData {
    pub local_branches: Vec<BranchInfo>,
    pub remote_branches: Vec<String>,
    pub tags: Vec<String>,
}

impl GitData {
    pub fn load_local() -> Self {
        Self {
            local_branches: branches::fetch_local_branches(),
            remote_branches: branches::fetch_remote_branches(),
            tags: branches::fetch_tags(),
        }
    }

    pub fn fetch_remotes() -> Result<(), String> {
        sync::fetch_remotes()
    }

    pub fn run(args: &[&str]) -> Result<String, String> {
        let full_command = format!("git {}", args.join(" "));

        let output = Command::new("git")
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute process: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(format!("Command '{}' failed: {}", full_command, stderr))
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

    pub fn is_ancestor(ancestor: &str, descendant: &str) -> Result<bool, String> {
        branches::is_ancestor(ancestor, descendant)
    }

    pub fn merge_into(target: &str, source: &str, strategy: MergeStrategy) -> Result<(), String> {
        branches::merge_into(target, source, strategy)
    }

    pub fn merge_with_strategy(branch: &str, strategy: MergeStrategy) -> Result<(), String> {
        branches::merge_with_strategy(branch, strategy)
    }
}
