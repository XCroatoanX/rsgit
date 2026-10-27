use super::commands::{get_output, run_command};
use super::types::{BranchInfo, GitTarget};
use crate::app::MergeStrategy;
use crate::git::GitData;
use rayon::prelude::*;
use std::process::Command;

pub fn fetch_local_branches() -> Vec<BranchInfo> {
    let output = Command::new("git")
        .args(["branch", "--format=%(HEAD)|%(refname:short)"])
        .output();

    let Ok(out) = output else { return Vec::new() };
    if !out.status.success() {
        return Vec::new();
    }

    let stdout = String::from_utf8_lossy(&out.stdout);

    let raw_branches: Vec<(bool, String)> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                let is_head = parts[0].trim() == "*";
                let name = parts[1].trim().to_string();
                Some((is_head, name))
            } else {
                None
            }
        })
        .collect();

    let mut branches: Vec<BranchInfo> = raw_branches
        .into_par_iter()
        .map(|(is_head, name)| {
            let (ahead, behind) = get_branch_counts(&name);
            BranchInfo {
                name,
                ahead,
                behind,
                is_head,
            }
        })
        .collect();

    branches.sort_by_key(|b| !b.is_head);

    branches
}

pub fn fetch_remote_branches() -> Vec<String> {
    get_output(&["branch", "-r", "--format=%(refname:short)"])
}

pub fn fetch_tags() -> Vec<String> {
    get_output(&["tag", "-l"])
}

pub fn create_entity(target: GitTarget, name: &str) -> Result<(), String> {
    let name = name.replace(' ', "-");
    let name = name.trim();

    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    let ref_spec = format!("HEAD:refs/heads/{}", name);

    let args: Vec<&str> = match target {
        GitTarget::LocalBranch => vec!["branch", name],
        GitTarget::RemoteBranch => vec!["push", "origin", &ref_spec],
        GitTarget::Tag => vec!["tag", name],
    };

    run_command(&args)
}

pub fn checkout_entity(target: GitTarget, name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Target name cannot be empty".to_string());
    }

    let args = match target {
        GitTarget::LocalBranch | GitTarget::Tag => vec!["checkout", name],
        GitTarget::RemoteBranch => {
            let local_name = name.split_once('/').map(|(_, r)| r).unwrap_or(name);
            vec!["checkout", "-b", local_name, name]
        }
    };

    run_command(&args)
}

pub fn delete_entity(target: GitTarget, name: &str) -> Result<(), String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Target name cannot be empty".to_string());
    }

    match target {
        GitTarget::LocalBranch => run_command(&["branch", "-D", name]),
        GitTarget::Tag => run_command(&["tag", "-d", name]),
        GitTarget::RemoteBranch => {
            let (remote, remote_branch) = name.split_once('/').unwrap_or(("origin", name));
            run_command(&["push", remote, "--delete", remote_branch])
        }
    }
}

pub fn rename_entity(target: GitTarget, old_name: &str, new_name: &str) -> Result<(), String> {
    let new_name = new_name.replace(' ', "-");
    let new_name = new_name.trim();

    if new_name.is_empty() {
        return Err("New name cannot be empty".to_string());
    }

    match target {
        GitTarget::LocalBranch => run_command(&["branch", "-m", old_name, new_name]),
        GitTarget::Tag => {
            run_command(&["tag", new_name, old_name])?;
            run_command(&["tag", "-d", old_name])
        }
        GitTarget::RemoteBranch => {
            let (remote, old_remote_branch) =
                old_name.split_once('/').unwrap_or(("origin", old_name));

            let refspec = format!("{}:refs/heads/{}", old_name, new_name);
            run_command(&["push", remote, &refspec])?;
            run_command(&["push", remote, "--delete", old_remote_branch])
        }
    }
}

fn get_branch_counts(branch: &str) -> (usize, usize) {
    let upstream_range = format!("{}...{}@{{upstream}}", branch, branch);
    if let Some(counts) = query_rev_list(&upstream_range) {
        return counts;
    }

    let fallback_range = format!("{}...origin/{}", branch, branch);
    if let Some(counts) = query_rev_list(&fallback_range) {
        return counts;
    }

    (0, 0)
}

fn query_rev_list(range: &str) -> Option<(usize, usize)> {
    if let Ok(stdout) = GitData::run(&["rev-list", "--left-right", "--count", range]) {
        let parts: Vec<&str> = stdout.split_whitespace().collect();
        if parts.len() == 2 {
            let ahead = parts[0].parse::<usize>().unwrap_or(0);
            let behind = parts[1].parse::<usize>().unwrap_or(0);
            return Some((ahead, behind));
        }
    }
    None
}

pub fn rebase_branch(branch: &str) -> Result<(), String> {
    let branch = branch.trim();
    if branch.is_empty() {
        return Err("Branch name cannot be empty".to_string());
    }
    run_command(&["rebase", branch])
}

pub fn current_branch() -> Result<String, String> {
    match GitData::run(&["symbolic-ref", "--quiet", "--short", "HEAD"]) {
        Ok(name) if !name.is_empty() => Ok(name),
        _ => Err("HEAD is detached or could not determine the current branch.".to_string()),
    }
}

fn working_tree_is_clean() -> Result<bool, String> {
    match GitData::run(&["status", "--porcelain", "--untracked-files=no"]) {
        Ok(stdout) => Ok(stdout.is_empty()),
        Err(err) => Err(err),
    }
}

pub fn is_ancestor(ancestor: &str, descendant: &str) -> bool {
    GitData::run(&["merge-base", "--is-ancestor", ancestor, descendant]).is_ok()
}

pub fn can_fast_forward_from(branch: &str) -> bool {
    is_ancestor("HEAD", branch)
}

pub fn can_fast_forward_into(branch: &str) -> bool {
    is_ancestor(branch, "HEAD")
}

pub fn merge_into(target: &str, source: &str, strategy: MergeStrategy) -> Result<(), String> {
    let target = target.trim();
    let source = source.trim();

    if target.is_empty() || source.is_empty() {
        return Err("Branch name cannot be empty".to_string());
    }

    if target == source {
        return Err(format!("'{}' is already the checked-out branch.", source));
    }

    if strategy == MergeStrategy::Cancel {
        return Ok(());
    }

    if !working_tree_is_clean()? {
        return Err(
            "Working tree has uncommitted changes. Commit or stash them before merging."
                .to_string(),
        );
    }

    run_command(&["checkout", target])?;

    let merge_result = match strategy {
        MergeStrategy::FastForward => run_command(&["merge", "--ff-only", source]),
        MergeStrategy::MergeCommit => run_command(&["merge", "--no-ff", source]),
        MergeStrategy::SquashUncommitted => run_command(&["merge", "--squash", source]),
        MergeStrategy::SquashCommit => run_command(&["merge", "--squash", source]).and_then(|_| {
            let msg = format!("Merge branch '{}' (squashed)", source);
            run_command(&["commit", "-m", &msg])
        }),
        MergeStrategy::Cancel => Ok(()),
    };

    match merge_result {
        Ok(()) => {
            if strategy != MergeStrategy::SquashUncommitted {
                run_command(&["checkout", source])?;
            }
            Ok(())
        }
        Err(err) => {
            if run_command(&["checkout", source]).is_ok() {
                Err(err)
            } else {
                Err(format!("{} (left on '{}' to resolve)", err, target))
            }
        }
    }
}

pub fn merge_with_strategy(branch: &str, strategy: MergeStrategy) -> Result<(), String> {
    let branch = branch.trim();
    if branch.is_empty() {
        return Err("Branch name cannot be empty".to_string());
    }

    match strategy {
        MergeStrategy::FastForward => run_command(&["merge", "--ff-only", branch]),
        MergeStrategy::MergeCommit => run_command(&["merge", "--no-ff", branch]),
        MergeStrategy::SquashUncommitted => run_command(&["merge", "--squash", branch]),
        MergeStrategy::SquashCommit => {
            run_command(&["merge", "--squash", branch])?;
            let msg = format!("Merge branch '{}' (squashed)", branch);
            run_command(&["commit", "-m", &msg])
        }
        MergeStrategy::Cancel => Ok(()),
    }
}
