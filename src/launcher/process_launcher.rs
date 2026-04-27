use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::{
    launcher::{LauncherProvider, LauncherType, LoadContext},
    loader::{
        resolve_icon_path,
        utils::{AppData, RawLauncher},
    },
    ui::widgets::RenderableChild,
};

#[derive(Debug, Clone, Copy, PartialEq, strum::VariantNames, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ProcessLauncherFunctions {
    Quit,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProcessLauncher {}

impl LauncherProvider for ProcessLauncher {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Process(ProcessLauncher {})
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &LoadContext,
        _opts: Arc<Value>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        Ok(vec![RenderableChild::App {
            inner: AppData {
                name: launcher.name.clone(),
                icon: resolve_icon_path("sherlock-process"),
                ..AppData::new()
            },
            launcher,
        }])
    }
}
