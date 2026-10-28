use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, EnumCount)]
pub enum ActiveBlock {
    #[default]
    Files,
    Branches,
    CommitHistory,
    Diff,
    Logs,
}

impl ActiveBlock {
    pub fn title(self) -> &'static str {
        match self {
            Self::Files => "Files",
            Self::Branches => "Branches",
            Self::CommitHistory => "Commit history",
            Self::Diff => "Diff",
            Self::Logs => "Git",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Files => Self::Branches,
            Self::Branches => Self::CommitHistory,
            Self::CommitHistory | Self::Diff | Self::Logs => Self::Files,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Files | Self::Diff | Self::Logs => Self::CommitHistory,
            Self::Branches => Self::Files,
            Self::CommitHistory => Self::Branches,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::iter().nth(index)
    }

    pub fn index(self) -> usize {
        Self::iter().position(|b| b == self).unwrap_or(0) + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum BranchTab {
    #[default]
    Local,
    Remote,
    Tags,
}

impl BranchTab {
    pub fn label(self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Remote => "Remote",
            Self::Tags => "Tags",
        }
    }

    pub fn next(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn previous(self) -> Self {
        let all = Self::iter().collect::<Vec<_>>();
        let pos = all.iter().position(|&x| x == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Popup {
    #[default]
    None,
    Help,
    About,
    Create,
    Rename,
    Delete,
    Merge,
    Rebase,
    Commit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommitField {
    #[default]
    Title,
    Message,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffContext {
    #[default]
    CommitHistory,
    Files,
    Branches,
}

impl From<ActiveBlock> for DiffContext {
    fn from(block: ActiveBlock) -> Self {
        match block {
            ActiveBlock::Files => Self::Files,
            ActiveBlock::Branches => Self::Branches,
            ActiveBlock::CommitHistory => Self::CommitHistory,
            ActiveBlock::Diff | ActiveBlock::Logs => Self::CommitHistory,
        }
    }
}
