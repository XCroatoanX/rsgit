use crate::app::{App, DeleteScope};
use crate::git::{GitData, GitTarget};
use crate::state::{BranchTab, CommitField, Popup};

impl App {
    pub fn open_create_popup(&mut self) {
        self.popup = Popup::Create;
        self.new_entity_input.clear();
        self.new_entity_cursor = 0;
        self.create_error_message = None;
    }

    pub fn close_create_popup(&mut self) {
        self.popup = Popup::None;
        self.new_entity_input.clear();
        self.create_error_message = None;
    }

    pub fn submit_create_entity(&mut self) {
        let target: GitTarget = self.branch_tab.into();

        match GitData::create_entity(target, &self.new_entity_input) {
            Ok(()) => {
                self.refresh_git();
                self.close_create_popup();
            }
            Err(err) => {
                self.create_error_message = Some(err);
            }
        }
    }

    pub fn open_delete_popup(&mut self) {
        if let Some(name) = self.get_selected_entity_name() {
            let has_local = self.git_data.local_branches.iter().any(|b| b.name == name);
            let has_remote = self.git_data.remote_branches.iter().any(|b| {
                let branch_name = b.split_once('/').map(|(_, rest)| rest).unwrap_or(b);
                branch_name == name || b == &name
            });

            self.delete_options = match self.branch_tab {
                BranchTab::Local => vec![
                    (
                        DeleteScope::LocalOnly,
                        "Delete Local Branch Only",
                        has_local,
                    ),
                    (
                        DeleteScope::Both,
                        "Delete Both (Local & Remote)",
                        has_local && has_remote,
                    ),
                    (
                        DeleteScope::RemoteOnly,
                        "Delete Remote Branch Only",
                        has_remote,
                    ),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
                BranchTab::Remote => vec![
                    (
                        DeleteScope::RemoteOnly,
                        "Delete Remote Branch Only",
                        has_remote,
                    ),
                    (
                        DeleteScope::Both,
                        "Delete Both (Local & Remote)",
                        has_local && has_remote,
                    ),
                    (
                        DeleteScope::LocalOnly,
                        "Delete Local Branch Only",
                        has_local,
                    ),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
                BranchTab::Tags => vec![
                    (DeleteScope::LocalOnly, "Delete Local Tag", true),
                    (DeleteScope::Cancel, "Cancel", true),
                ],
            };

            self.delete_selected_index = self
                .delete_options
                .iter()
                .position(|(_, _, enabled)| *enabled)
                .unwrap_or(0);
            self.delete_error_message = None;
            self.popup = Popup::Delete;
        }
    }

    pub fn confirm_delete_selected_entity(&mut self) {
        let Some(name) = self.get_selected_entity_name() else {
            return;
        };
        let Some((scope, _, enabled)) = self.delete_options.get(self.delete_selected_index) else {
            return;
        };
        if !enabled {
            return;
        }

        self.popup = Popup::None;
        let result = match (self.branch_tab, scope) {
            (BranchTab::Tags, DeleteScope::LocalOnly) => {
                GitData::delete_entity(GitTarget::Tag, &name)
            }
            (_, DeleteScope::LocalOnly) => GitData::delete_entity(GitTarget::LocalBranch, &name),
            (_, DeleteScope::RemoteOnly) => GitData::delete_entity(GitTarget::RemoteBranch, &name),
            (_, DeleteScope::Both) => GitData::delete_entity(GitTarget::LocalBranch, &name)
                .and(GitData::delete_entity(GitTarget::RemoteBranch, &name)),
            (_, DeleteScope::Cancel) => Ok(()),
        };

        if let Err(err) = result {
            self.error_message = Some(err);
        } else {
            self.selected_branch_index = 0;
            self.refresh_git();
        }
    }

    pub fn open_rename_popup(&mut self) {
        if self.branch_tab == BranchTab::Remote {
            self.error_message = Some(
                "Renaming remote branches directly is not supported by Git. Create a new remote branch and delete the old one instead.".into(),
            );
            return;
        }

        if let Some(name) = self.get_selected_entity_name() {
            self.rename_input = name.clone();
            self.rename_cursor = name.chars().count();
            self.rename_warning = if self.git_data.remote_branches.iter().any(|branch| {
                let branch_name = branch
                    .split_once('/')
                    .map(|(_, rest)| rest)
                    .unwrap_or(branch);
                branch_name == name || branch == &name
            }) {
                Some(
                    "Renaming only affects your local branch. Remote branch will not be changed."
                        .into(),
                )
            } else {
                None
            };
            self.rename_error_message = None;
            self.popup = Popup::Rename;
        }
    }

    pub fn submit_rename_selected_entity(&mut self) {
        if let Some(old_name) = self.get_selected_entity_name() {
            let target: GitTarget = self.branch_tab.into();
            match GitData::rename_entity(target, &old_name, &self.rename_input) {
                Ok(()) => {
                    self.popup = Popup::None;
                    self.rename_input.clear();
                    self.refresh_git();
                }
                Err(err) => {
                    self.rename_error_message = Some(err);
                }
            }
        }
    }

    pub fn open_commit_popup(&mut self) {
        if self.active_action.is_some() || self.active_sync.is_some() {
            self.error_message = Some("Another git action is running. Wait for it to finish.".into());
            return;
        }

        if self.staged_file_count() == 0 {
            self.error_message =
                Some("No staged changes to commit. Stage files first (Space or 'a').".into());
            return;
        }

        self.popup = Popup::Commit;
        self.commit_title.clear();
        self.commit_title_cursor = 0;
        self.commit_message.clear();
        self.commit_message_cursor = 0;
        self.commit_field = CommitField::Title;
        self.commit_error_message = None;
    }

    pub fn close_commit_popup(&mut self) {
        self.popup = Popup::None;
        self.commit_title.clear();
        self.commit_message.clear();
        self.commit_error_message = None;
    }

    pub fn submit_commit(&mut self) {
        if self.active_action.is_some() || self.active_sync.is_some() {
            self.commit_error_message = Some("Another git action is running.".to_string());
            return;
        }

        if self.commit_title.trim().is_empty() {
            self.commit_error_message = Some("Commit title cannot be empty.".to_string());
            return;
        }

        if self.staged_file_count() == 0 {
            self.commit_error_message = Some("No staged changes to commit.".to_string());
            return;
        }

        match GitData::commit(self.commit_title.trim(), self.commit_message.trim()) {
            Ok(()) => {
                self.close_commit_popup();
                self.refresh_git();
            }
            Err(err) => {
                self.commit_error_message = Some(err);
            }
        }
    }
}
