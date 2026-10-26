use std::process::Command;

pub fn run_command(args: &[&str]) -> Result<(), String> {
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

pub fn get_output(args: &[&str]) -> Vec<String> {
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
