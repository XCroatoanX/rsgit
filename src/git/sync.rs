use std::process::Command;

pub fn fetch_remotes() -> Result<(), String> {
    let output = Command::new("git")
        .args(["fetch", "--prune", "--tags"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn pull_branch(branch: &str) -> Result<(), String> {
    let output = Command::new("git")
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
    let output = Command::new("git")
        .args(["push", "origin", branch])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
