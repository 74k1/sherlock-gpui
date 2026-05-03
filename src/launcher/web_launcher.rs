use crate::{
    launcher::{LauncherProvider, LauncherType, app_launcher::app_data::AppData},
    loader::utils::{PriorityGuard, RawLauncher},
    ui::widgets::RenderableChild,
    utils::errors::SherlockMessage,
};
use gpui::SharedString;
use serde::Deserialize;
use serde_json::Value;

/// The following arguments are available to users:
/// - `engine`: The engine to be used for the query
/// - `browser`: The browser to be used for opening the query, defaults
/// - `display_name`: The display name for this tile, replacing `{keyword}` with query
#[derive(Clone, Debug, Deserialize)]
pub struct WebLauncher {
    #[serde(rename = "search_engine")]
    pub engine: String,
    pub browser: Option<String>,
}

impl LauncherProvider for WebLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        match serde_json::from_value::<WebLauncher>(raw.args.as_ref().clone()) {
            Ok(launcher) => LauncherType::Web(launcher),
            Err(_) => LauncherType::Empty,
        }
    }
    fn objects(
        &self,
        launcher: std::sync::Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        opts: std::sync::Arc<serde_json::Value>,
        _messages: &mut Vec<SherlockMessage>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let name: Option<SharedString> = opts
            .get("display_name")
            .and_then(Value::as_str)
            .map(String::from)
            .map(SharedString::from);

        let inner = AppData {
            name,
            icon: launcher.icon.clone(),
            priority: PriorityGuard::new_with_launcher(&launcher, 0),
            ..AppData::new()
        };

        Ok(vec![RenderableChild::App { launcher, inner }])
    }
}
