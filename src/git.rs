use rayon::prelude::*;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInfo {
    pub name: String,
    pub ahead: usize,
    pub behind: usize,
    pub is_head: bool,
}

impl BranchInfo {
    pub fn display_name(&self) -> String {
        let prefix = if self.is_head { "* " } else { "  " };
        let mut status = String::new();

        if self.ahead > 0 {
            status.push_str(&format!(" ↑{}", self.ahead));
        }
        if self.behind > 0 {
            status.push_str(&format!(" ↓{}", self.behind));
        }

        format!("{}{}{}", prefix, self.name, status)
    }
}

#[derive(Debug, Clone, Default)]
pub struct GitData {
    pub local_branches: Vec<BranchInfo>,
    pub remote_branches: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitLine {
    pub graph: String,
    pub hash: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitTarget {
    LocalBranch,
    RemoteBranch,
    Tag,
}

impl GitData {
    pub fn fetch_branches() -> Self {
        let _ = Self::fetch_remotes();
        Self {
            local_branches: Self::fetch_local_branches(),
            remote_branches: Self::get_output(&["branch", "-r", "--format=%(refname:short)"]),
            tags: Self::get_output(&["tag", "-l"]),
        }
    }

    pub fn fetch_remotes() -> Result<(), String> {
        let output = std::process::Command::new("git")
            .args(["fetch", "--prune", "--tags"])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

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
                let (ahead, behind) = Self::get_branch_counts(&name);
                BranchInfo {
                    name,
                    ahead,
                    behind,
                    is_head,
                }
            })
            .collect()
    }

    fn get_branch_counts(branch: &str) -> (usize, usize) {
        let upstream_range = format!("{}...{}@{{upstream}}", branch, branch);
        if let Some(counts) = Self::query_rev_list(&upstream_range) {
            return counts;
        }

        let fallback_range = format!("{}...origin/{}", branch, branch);
        if let Some(counts) = Self::query_rev_list(&fallback_range) {
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

    fn get_output(args: &[&str]) -> Vec<String> {
        let output = Command::new("git").args(args).output();

        match output {
            Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            _ => vec![],
        }
    }

    pub fn fetch_history(target: &str) -> Vec<CommitLine> {
        let lines = Self::get_output(&[
            "log",
            "--graph",
            "--no-decorate",
            "--pretty=format:%x01%h%x1f%an%x1f%s",
            "-50",
            target,
        ]);

        lines
            .into_iter()
            .map(|line| {
                if let Some((graph, commit)) = line.split_once('\x01') {
                    let mut fields = commit.splitn(3, '\x1f');
                    CommitLine {
                        graph: graph.to_string(),
                        hash: fields.next().map(str::to_string),
                        author: fields.next().map(str::to_string),
                        subject: fields.next().map(str::to_string),
                    }
                } else {
                    CommitLine {
                        graph: line,
                        hash: None,
                        author: None,
                        subject: None,
                    }
                }
            })
            .collect()
    }

    pub fn fetch_checked_out_history() -> Vec<CommitLine> {
        let output = Command::new("git")
            .args(["symbolic-ref", "--quiet", "--short", "HEAD"])
            .output();

        let Ok(output) = output else {
            return Vec::new();
        };

        if !output.status.success() {
            return Vec::new();
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch.is_empty() {
            Vec::new()
        } else {
            Self::fetch_history(&branch)
        }
    }

    pub fn fetch_commit_diff(commit: &str) -> Vec<String> {
        let output = Command::new("git")
            .args(["show", "--format=", "--no-ext-diff", "--no-color", commit])
            .output();

        match output {
            Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(str::to_string)
                .collect(),
            _ => Vec::new(),
        }
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

        Self::run_command(&args)
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

        Self::run_command(&args)
    }
    pub fn delete_entity(target: GitTarget, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("Target name cannot be empty".to_string());
        }

        match target {
            GitTarget::LocalBranch => Self::run_command(&["branch", "-D", name]),
            GitTarget::Tag => Self::run_command(&["tag", "-d", name]),
            GitTarget::RemoteBranch => {
                if let Some((remote, remote_branch)) = name.split_once('/') {
                    Self::run_command(&["push", remote, "--delete", remote_branch])
                } else {
                    Self::run_command(&["push", "origin", "--delete", name])
                }
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
            GitTarget::LocalBranch => Self::run_command(&["branch", "-m", old_name, new_name]),
            GitTarget::Tag => {
                Self::run_command(&["tag", new_name, old_name])?;
                Self::run_command(&["tag", "-d", old_name])
            }
            GitTarget::RemoteBranch => {
                let (remote, old_remote_branch) =
                    old_name.split_once('/').unwrap_or(("origin", old_name));

                let refspec = format!("{}:refs/heads/{}", old_name, new_name);
                Self::run_command(&["push", remote, &refspec])?;
                Self::run_command(&["push", remote, "--delete", old_remote_branch])
            }
        }
    }
    fn run_command(args: &[&str]) -> Result<(), String> {
        let output = Command::new("git").args(args).output();
        match output {
            Ok(out) => {
                if out.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    Err(stderr.trim().to_string())
                }
            }
            Err(err) => Err(format!("Failed to execute git command: {}", err)),
        }
    }
    pub fn pull_branch(branch: &str) -> Result<(), String> {
        let output = std::process::Command::new("git")
            .args(["pull", "origin", branch])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn push_branch(branch: &str) -> Result<(), String> {
        let output = std::process::Command::new("git")
            .args(["push", "origin", branch])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
