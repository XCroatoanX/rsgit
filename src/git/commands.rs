use crate::git::GitData;

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
