use std::process::Command;
use rayon::prelude::*;

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

impl GitData {
    pub fn fetch_branches() -> Self {
        Self {
            local_branches: Self::fetch_local_branches(),
            remote_branches: Self::get_output(&["branch", "-r", "--format=%(refname:short)"]),
            tags: Self::get_output(&["tag", "-l"]),
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
        let range = format!("{}...{}@{{upstream}}", branch, branch);
        let output = Command::new("git")
            .args(["rev-list", "--left-right", "--count", &range])
            .output();

        if let Ok(out) = output {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = stdout.split_whitespace().collect();
                if parts.len() == 2 {
                    let ahead = parts[0].parse::<usize>().unwrap_or(0);
                    let behind = parts[1].parse::<usize>().unwrap_or(0);
                    return (ahead, behind);
                }
            }
        }

        (0, 0)
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
}
