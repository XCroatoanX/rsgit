use crate::git::GitData;

pub fn fetch_remotes() -> Result<(), String> {
    GitData::run(&["fetch", "--prune", "--tags"]).map(|_| ())
}

pub fn pull_branch(branch: &str) -> Result<(), String> {
    GitData::run(&["pull", "origin", branch]).map(|_| ())
}

pub fn push_branch(branch: &str) -> Result<(), String> {
    GitData::run(&["push", "origin", branch]).map(|_| ())
}
