pub mod branches;
pub mod commit_history;
pub mod diff;
pub mod files;
pub mod footer;
pub mod logs;

pub use branches::BranchesComponent;
pub use commit_history::CommitHistoryComponent;
pub use diff::DiffComponent;
pub use footer::FooterComponent;
pub use logs::LogsComponent;
pub use staged_files::FilesComponent;
