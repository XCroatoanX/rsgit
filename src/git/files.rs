use super::commands::{git_command, run_command};
use super::types::ChangedFile;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffRow {
    Folder {
        name: String,
        path: String,
        depth: usize,
    },
    File {
        file: ChangedFile,
        depth: usize,
    },
}

pub fn fetch_changed_files() -> Vec<ChangedFile> {
    let output = git_command()
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output();

    let Ok(output) = output else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let fields: Vec<&str> = stdout.split('\0').collect();
    let mut files = Vec::new();

    let mut i = 0;
    while i < fields.len() {
        let field = fields[i];
        i += 1;
        if field.len() < 3 {
            continue;
        }

        let index = field.as_bytes()[0] as char;
        let worktree = field.as_bytes()[1] as char;
        if index == 'R' || index == 'C' {
            i += 1;
        }

        files.push(ChangedFile {
            index,
            worktree,
            path: field[3..].to_string(),
        });
    }

    files
}

pub fn stage_file(path: &str) -> Result<(), String> {
    run_command(&["add", "-A", "--", path])
}

pub fn unstage_file(path: &str) -> Result<(), String> {
    run_command(&["restore", "--staged", "--", path])
}

pub fn commit(title: &str, message: &str) -> Result<(), String> {
    if message.trim().is_empty() {
        run_command(&["commit", "-m", title])
    } else {
        run_command(&["commit", "-m", title, "-m", message])
    }
}

pub fn stage_all() -> Result<(), String> {
    run_command(&["add", "-A"])
}

pub fn unstage_all() -> Result<(), String> {
    run_command(&["reset"])
}

pub fn fetch_changed_file_diff(path: &str, untracked: bool) -> Vec<String> {
    let output = if untracked {
        git_command()
            .args(["diff", "--no-index", "--no-color", "/dev/null", path])
            .output()
    } else {
        git_command()
            .args(["diff", "HEAD", "--", path])
            .output()
    };

    match output {
        Ok(output) if matches!(output.status.code(), Some(0) | Some(1)) => {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(str::to_string)
                .collect()
        }
        _ => Vec::new(),
    }
}

pub fn build_rows(files: &[ChangedFile], collapsed: &HashSet<String>) -> Vec<DiffRow> {
    fn flatten(
        prefix: &str,
        files: &[&ChangedFile],
        collapsed: &HashSet<String>,
        rows: &mut Vec<DiffRow>,
        depth: usize,
    ) {
        let mut folders: BTreeMap<&str, Vec<&ChangedFile>> = BTreeMap::new();
        let mut direct: Vec<&ChangedFile> = Vec::new();

        for &file in files {
            let rest = if prefix.is_empty() {
                file.path.as_str()
            } else {
                match file.path.strip_prefix(prefix) {
                    Some(rest) => rest.trim_start_matches('/'),
                    None => file.path.as_str(),
                }
            };

            if let Some((head, _)) = rest.split_once('/') {
                folders.entry(head).or_default().push(file);
            } else {
                direct.push(file);
            }
        }

        for (name, children) in folders {
            let path = if prefix.is_empty() {
                (*name).to_string()
            } else {
                format!("{prefix}/{name}")
            };
            rows.push(DiffRow::Folder {
                name: (*name).to_string(),
                path: path.clone(),
                depth,
            });
            if !collapsed.contains(&path) {
                flatten(&path, &children, collapsed, rows, depth + 1);
            }
        }

        for file in direct {
            rows.push(DiffRow::File {
                file: (*file).clone(),
                depth,
            });
        }
    }

    let refs: Vec<&ChangedFile> = files.iter().collect();
    let mut rows = Vec::new();
    flatten("", &refs, collapsed, &mut rows, 0);
    rows
}