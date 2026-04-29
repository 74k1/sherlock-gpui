use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::launcher::variant_type::InnerFunction;
use crate::launcher::{ExecEffect, LauncherProvider, LauncherType};
use crate::loader::resolve_icon_path;
use crate::loader::utils::{AppData, ApplicationAction, RawLauncher};
use crate::ui::launcher::context_menu::ContextMenuAction;
use crate::ui::widgets::RenderableChild;
use crate::utils::config::ConfigGuard;
use crate::utils::errors::SherlockMessage;
use crate::utils::errors::types::{DirAction, FileAction, SherlockErrorType};
use crate::utils::paths;
use crate::{define_inner_functions, ensure_func, sherlock_msg};
use gpui::App;

define_inner_functions! {
    pub enum DebugFunctions {
        ClearCache,
        ClearAppCounts,
        ClearErrors,
        InsertTestErrors,
    }
}

#[derive(Clone, Debug)]
pub struct DebugLauncher {}

impl LauncherProvider for DebugLauncher {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Debug(DebugLauncher {})
    }
    fn objects(
        &self,
        launcher: std::sync::Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        _opts: std::sync::Arc<serde_json::Value>,
        _messages: &mut Vec<SherlockMessage>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        Ok(vec![RenderableChild::App {
            launcher: launcher.clone(),
            inner: AppData {
                name: Some("Debug".into()),
                search_string: "clear cache;debug;app count".into(),
                icon: resolve_icon_path("sherlock-devtools"),
                actions: Arc::from([
                    Arc::new(ContextMenuAction::App(ApplicationAction {
                        name: Some("Clear Cache".into()),
                        method: "inner.clear_cache".into(),
                        icon: resolve_icon_path("sherlock-process"),
                        exit: launcher.exit,
                        ..Default::default()
                    })),
                    Arc::new(ContextMenuAction::App(ApplicationAction {
                        name: Some("Reset App Count".into()),
                        method: "inner.clear_app_counts".into(),
                        icon: resolve_icon_path("sherlock-process"),
                        exit: launcher.exit,
                        ..Default::default()
                    })),
                    Arc::new(ContextMenuAction::App(ApplicationAction {
                        name: Some("Clear Error Messages".into()),
                        method: "inner.clear_errors".into(),
                        icon: resolve_icon_path("sherlock-process"),
                        exit: launcher.exit,
                        ..Default::default()
                    })),
                    Arc::new(ContextMenuAction::App(ApplicationAction {
                        name: Some("Insert Test Errors".into()),
                        method: "inner.insert_test_errors".into(),
                        icon: resolve_icon_path("sherlock-devtools"),
                        exit: launcher.exit,
                        ..Default::default()
                    })),
                ]),
                ..AppData::new()
            },
        }])
    }
    fn execute_function(
        &self,
        func: super::variant_type::InnerFunction,
        _child: &RenderableChild,
        _variables: &[(gpui::SharedString, gpui::SharedString)],
        _cx: &mut App,
    ) -> Result<ExecEffect, crate::utils::errors::SherlockMessage> {
        let func = ensure_func!(func, InnerFunction::Debug);

        match func {
            DebugFunctions::ClearCache => DebugFunctions::clear_cache()?,
            DebugFunctions::ClearAppCounts => DebugFunctions::clear_app_counts()?,
            DebugFunctions::ClearErrors => return Ok(ExecEffect::ClearMessages),
            DebugFunctions::InsertTestErrors => {
                return Ok(ExecEffect::InsertMessages(vec![
                    sherlock_msg!(
                        Info,
                        SherlockErrorType::Preview,
                        "This is a test info message"
                    ),
                    sherlock_msg!(
                        Warning,
                        SherlockErrorType::Preview,
                        "This is a test warning message"
                    ),
                    sherlock_msg!(
                        Error,
                        SherlockErrorType::Preview,
                        "This is a test error message"
                    ),
                ]));
            }
        }

        Ok(ExecEffect::None)
    }
}

impl DebugFunctions {
    fn clear_cache() -> Result<(), SherlockMessage> {
        let cache_dir = paths::get_cache_dir()?;
        let app_cache = ConfigGuard::read().map(|c| c.caching.cache.clone())?;

        Self::remove_dir_safe(cache_dir)?;
        Self::remove_file_safe(app_cache)?;

        Ok(())
    }
    fn clear_app_counts() -> Result<(), SherlockMessage> {
        let counts = paths::get_data_dir()?.join("counts.bin");
        Self::remove_file_safe(counts)
    }

    /// Safely removes a file from the file system, skipping files if they dont exist.
    #[inline]
    fn remove_file_safe(path: PathBuf) -> Result<(), SherlockMessage> {
        if !path.exists() {
            return Ok(());
        }

        fs::remove_file(&path).map_err(|e| {
            sherlock_msg!(
                Warning,
                SherlockErrorType::FileError(FileAction::Remove, path),
                e
            )
        })
    }
    /// Safely removes a directory from the file system, skipping directories if they dont exist.
    #[inline]
    fn remove_dir_safe(path: PathBuf) -> Result<(), SherlockMessage> {
        if !path.exists() {
            return Ok(());
        }

        fs::remove_dir_all(&path).map_err(|e| {
            sherlock_msg!(
                Warning,
                SherlockErrorType::DirError(DirAction::Remove, path),
                e
            )
        })
    }
}
