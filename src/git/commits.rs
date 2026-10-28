use super::commands::{get_output, git_command};
use super::types::CommitLine;

const HISTORY_LIMIT: usize = 50;

pub fn fetch_history(target: &str) -> Vec<CommitLine> {
    let limit = format!("-{HISTORY_LIMIT}");
    let lines = get_output(&[
        "log",
        "--graph",
        "--no-decorate",
        "--pretty=format:%x01%h%x1f%an%x1f%s",
        &limit,
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
    let output = git_command()
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
        fetch_history(&branch)
    }
}

pub fn fetch_commit_diff(commit: &str) -> Vec<String> {
    let output = git_command()
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
