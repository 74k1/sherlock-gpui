use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::{
    launcher::{LauncherProvider, LauncherType, LoadContext},
    loader::{application_loader::ApplicationLoader, utils::RawLauncher},
    ui::widgets::RenderableChild,
    utils::errors::SherlockMessage,
};

pub mod app_data;
pub mod app_serde;

/// The following arguments are available to users:
/// - `use_keywords`: Whether the search should use the keywords or only the app name
#[derive(Clone, Debug, Deserialize)]
pub struct AppLauncher {
    #[serde(default)]
    pub use_keywords: bool,
}

impl LauncherProvider for AppLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        match serde_json::from_value::<AppLauncher>(raw.args.as_ref().clone()) {
            Ok(launcher) => LauncherType::Apps(launcher),
            Err(_) => LauncherType::Empty,
        }
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        ctx: &LoadContext,
        _opts: Arc<Value>,
        _messages: &mut Vec<SherlockMessage>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        ApplicationLoader::load_applications(Arc::clone(&launcher), &ctx.counts, self.use_keywords)
            .map(|apps| {
                Arc::unwrap_or_clone(apps)
                    .into_iter()
                    .map(|inner| RenderableChild::App {
                        launcher: Arc::clone(&launcher),
                        inner,
                    })
                    .collect()
            })
    }
}
