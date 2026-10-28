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
    command.env("GIT_TERMINAL_PROMPT", "0");
    command
}

pub fn run_command(args: &[&str]) -> Result<(), String> {
    GitData::run(args).map(|_| ())
}

pub fn run_command_with_env(args: &[&str], envs: &[(&str, &str)]) -> Result<(), String> {
    let full_command = format!("git {}", args.join(" "));

    let output = git_command()
        .args(args)
        .envs(envs.iter().map(|(k, v)| (*k, *v)))
        .output()
        .map_err(|e| format!("Failed to execute process: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!("Command '{}' failed: {}", full_command, stderr))
    }
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

pub fn get_identity() -> (Option<String>, Option<String>) {
    let name = get_output(&["config", "user.name"]).first().cloned();
    let email = get_output(&["config", "user.email"]).first().cloned();
    (name, email)
}

pub fn remote_url() -> String {
    get_output(&["remote", "get-url", "origin"])
        .into_iter()
        .next()
        .unwrap_or_default()
}

pub fn askpass_script() -> Result<PathBuf, String> {
    let path = std::env::temp_dir().join(format!(
        "rsgit_askpass_{}.{}",
        std::process::id(),
        if cfg!(windows) { "cmd" } else { "sh" }
    ));

    #[cfg(windows)]
    let content = "@echo off\r\necho %1 | findstr /I \"username\" >nul\r\nif %errorlevel%==0 (\r\n  echo %RSGIT_ASKPASS_USERNAME%\r\n) else (\r\n  echo %RSGIT_ASKPASS_PASSWORD%\r\n)\r\n".to_string();

    #[cfg(unix)]
    let content = "#!/bin/sh\ncase \"$1\" in\n  *[Uu]sername*) echo \"$RSGIT_ASKPASS_USERNAME\" ;;\n  *) echo \"$RSGIT_ASKPASS_PASSWORD\" ;;\nesac\n".to_string();

    std::fs::write(&path, content).map_err(|e| format!("Failed to write askpass helper: {}", e))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700));
    }

    Ok(path)
}
