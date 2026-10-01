use color_eyre::Result;
use std::env;
use std::io::{self, Write};
use std::path::{PathBuf};
use std::process::{Command, Stdio};

pub enum StartupAction {
    Continue,
    ExitSuccess,
}

fn expand_path(raw_path: &str) -> PathBuf {
    let path = if raw_path == "~" || raw_path.starts_with("~/") || raw_path.starts_with("~\\") {
        if let Some(home_dir) = dirs::home_dir() {
            home_dir.join(&raw_path[2..])
        } else {
            PathBuf::from(raw_path)
        }
    } else {
        PathBuf::from(raw_path)
    };
    dunce::canonicalize(&path).unwrap_or(path)
}

pub fn ensure_git_repo(target_dir: Option<&str>) -> Result<StartupAction> {
    if let Some(raw_path) = target_dir {
        let path = expand_path(raw_path);

        if !path.exists() {
            eprintln!("Error: Directory '{}' does not exist.", path.display());
            std::process::exit(1);
        }

        if !path.is_dir() {
            eprintln!("Error: Path '{}' is not a directory.", path.display());
            std::process::exit(1);
        }

        env::set_current_dir(&path)?;
    }

    let git_check = Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match git_check {
        Ok(status) if status.success() => {}
        _ => {
            eprintln!("Error: Git is not installed or not available in your PATH.");
            std::process::exit(1);
        }
    }

    let is_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false);

    if is_repo {
        return Ok(StartupAction::Continue);
    }

    let current_dir = env::current_dir()?;
    print!(
        "Directory '{}' is not a Git repository. Initialize one here? (y/N): ",
        current_dir.display()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim().to_lowercase();

    if choice == "y" || choice == "yes" {
        let init_status = Command::new("git").arg("init").status()?;

        if init_status.success() {
            println!("Initialized empty Git repository.");
            Ok(StartupAction::Continue)
        } else {
            eprintln!("Error: Failed to run 'git init'.");
            std::process::exit(1);
        }
    } else {
        println!("Aborted. Not in a Git repository.");
        Ok(StartupAction::ExitSuccess)
    }
}