#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInfo {
    pub name: String,
    pub ahead: usize,
    pub behind: usize,
    pub is_head: bool,
}

impl BranchInfo {
    pub fn display_name(&self) -> String {
        let prefix = if self.is_head { "* " } else { "  " };
        let mut status = String::new();

        if self.ahead > 0 {
            status.push_str(&format!(" ↑{}", self.ahead));
        }
        if self.behind > 0 {
            status.push_str(&format!(" ↓{}", self.behind));
        }

        format!("{}{}{}", prefix, self.name, status)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedFile {
    pub index: char,
    pub worktree: char,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitLine {
    pub graph: String,
    pub hash: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitTarget {
    LocalBranch,
    RemoteBranch,
    Tag,
}
