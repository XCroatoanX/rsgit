use super::commands;
use crate::git::GitData;

pub fn fetch_remotes() -> Result<(), String> {
    if GitData::run(&["remote"])?.is_empty() {
        return Ok(());
    }

    GitData::run(&["fetch", "--prune", "--tags"]).map(|_| ())
}

pub fn pull_branch(branch: &str) -> Result<(), String> {
    GitData::run(&["pull", "origin", branch]).map(|_| ())
}

pub fn push_branch(branch: &str) -> Result<(), String> {
    GitData::run(&["push", "origin", branch]).map(|_| ())
}

pub fn pull_branch_with_auth(branch: &str, username: &str, password: &str) -> Result<(), String> {
    run_with_auth(&["pull", "origin", branch], username, password)
}

pub fn push_branch_with_auth(branch: &str, username: &str, password: &str) -> Result<(), String> {
    run_with_auth(&["push", "origin", branch], username, password)
}

fn run_with_auth(args: &[&str], username: &str, password: &str) -> Result<(), String> {
    let script = commands::askpass_script()?;
    let script_path = script.to_str().unwrap_or_default().to_string();
    let envs = [
        ("GIT_ASKPASS", script_path.as_str()),
        ("SSH_ASKPASS", script_path.as_str()),
        ("SSH_ASKPASS_REQUIRE", "force"),
        ("RSGIT_ASKPASS_USERNAME", username),
        ("RSGIT_ASKPASS_PASSWORD", password),
    ];
    let result = commands::run_command_with_env(args, &envs);
    let _ = std::fs::remove_file(&script);
    result
}
