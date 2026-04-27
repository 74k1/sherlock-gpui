use std::path::PathBuf;

use crate::launcher::{LauncherProvider, LauncherType};
use crate::loader::utils::RawLauncher;
use crate::ui::widgets::RenderableChild;
use crate::utils::files::{expand_path, home_dir};

#[derive(Clone, Debug)]
pub struct ThemePicker {}

impl LauncherProvider for ThemePicker {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Theme(ThemePicker {})
    }
    fn objects(
        &self,
        _launcher: std::sync::Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        opts: std::sync::Arc<serde_json::Value>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let path_str = opts
            .get("path")
            .and_then(|p| p.as_str())
            .unwrap_or("~/.config/sherlock/themes/");

        // expand homedir
        let home = home_dir()?;
        let path = expand_path(path_str, &home);

        if path.exists() && path.is_dir() {
            Ok(vec![])
        } else {
            // default sherlock themes
            Ok(vec![])
        }
    }
}
