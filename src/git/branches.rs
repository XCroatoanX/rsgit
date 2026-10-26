use super::commands::{get_output, run_command};
use super::types::{BranchInfo, GitTarget};
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

    raw_branches
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
        .collect()
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
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", range])
        .output();

    if let Ok(out) = output
        && out.status.success()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let parts: Vec<&str> = stdout.split_whitespace().collect();
        if parts.len() == 2 {
            let ahead = parts[0].parse::<usize>().unwrap_or(0);
            let behind = parts[1].parse::<usize>().unwrap_or(0);
            return Some((ahead, behind));
        }
    }

    None
}
