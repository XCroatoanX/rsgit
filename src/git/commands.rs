use crate::git::GitData;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static REPO_ROOT: OnceLock<PathBuf> = OnceLock::new();

pub fn repo_root() -> PathBuf {
    REPO_ROOT
        .get_or_init(|| {
            Command::new("git")
                .args(["rev-parse", "--show-toplevel"])
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| {
                    let root = String::from_utf8_lossy(&output.stdout);
                    let root = root.trim();
                    if root.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(root))
                    }
                })
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        })
        .clone()
}

pub fn git_command() -> Command {
    let mut command = Command::new("git");
    command.current_dir(repo_root());
    command
}

pub fn run_command(args: &[&str]) -> Result<(), String> {
    GitData::run(args).map(|_| ())
}

pub fn get_output(args: &[&str]) -> Vec<String> {
    match GitData::run(args) {
        Ok(stdout) => stdout
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => vec![],
    }
}
