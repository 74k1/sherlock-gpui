use std::{collections::HashSet, fs, path::Path, time::SystemTime};

use crate::{
    loader::application_loader::get_applications_dir,
    sherlock_msg,
    utils::{
        config::ConfigGuard,
        errors::{
            SherlockMessage,
            types::{DirAction, SherlockErrorType},
        },
    },
};

/// **Unfinished**
/// This struct aims at providing an audit function to check for config file changes and
/// application data changes. This should be run on every startup.
///
/// TODO:
/// Add functionality for .desktop files.
/// Add audit file that contains last audit time.
///
pub struct ConfigWatcher {
    latest_audit: SystemTime,
    root_dir: Box<Path>,
}

impl ConfigWatcher {
    pub fn new(root_dir: Box<Path>) -> Self {
        Self {
            latest_audit: SystemTime::now(),
            root_dir,
        }
    }

    pub fn audit(&mut self) -> Result<HashSet<ConfigFileChange>, SherlockMessage> {
        let current_audit_time = SystemTime::now();
        let since = self.latest_audit;

        // get entries
        let entries = std::fs::read_dir(&self.root_dir).map_err(|e| {
            sherlock_msg!(
                Warning,
                SherlockErrorType::DirError(DirAction::Read, self.root_dir.to_path_buf()),
                e
            )
        })?;

        let files = ConfigGuard::read()
            .map(|c| c.files.clone())
            .unwrap_or_default();

        // collect out-of-date entries
        let mut changes: HashSet<ConfigFileChange> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .metadata()
                    .and_then(|m| m.modified())
                    .map(|modified| entry.path().is_file() && modified > since)
                    .unwrap_or(false)
            })
            .map(|entry| {
                let path_buf = entry.path().to_path_buf();
                match path_buf {
                    _ if path_buf == files.config => ConfigFileChange::Config,
                    _ if path_buf == files.fallback => ConfigFileChange::Fallback,
                    _ if path_buf == files.alias => ConfigFileChange::Alias,
                    _ if path_buf == files.ignore => ConfigFileChange::Ignore,
                    _ if path_buf == files.actions => ConfigFileChange::Actions,
                    _ => ConfigFileChange::Other,
                }
            })
            .collect();

        // check desktop files
        let app_dirs = get_applications_dir();
        let apps_have_changed = app_dirs
            .into_iter()
            .any(|dir| any_file_modified_after(&dir, since).is_ok_and(|c| c));
        if apps_have_changed {
            changes.insert(ConfigFileChange::Apps);
        }

        self.latest_audit = current_audit_time;

        Ok(changes)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum ConfigFileChange {
    Actions,
    Alias,
    Apps,
    Config,
    Ignore,
    Fallback,
    Other,
}

fn any_file_modified_after(dir: &Path, since: SystemTime) -> Result<bool, SherlockMessage> {
    if !dir.exists() {
        return Err(sherlock_msg!(
            Warning,
            SherlockErrorType::DirError(DirAction::Find, dir.to_path_buf()),
            "Directory does not exist."
        ));
    }

    let any_file = fs::read_dir(dir)
        .map_err(|e| {
            sherlock_msg!(
                Warning,
                SherlockErrorType::DirError(DirAction::Read, dir.to_path_buf()),
                e
            )
        })?
        .filter_map(Result::ok)
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            let is_desktop = e
                .path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("desktop"));

            (meta.is_file() && is_desktop)
                .then(|| meta.modified().ok())
                .flatten()
        })
        .any(|m| m > since);

    Ok(any_file)
}
