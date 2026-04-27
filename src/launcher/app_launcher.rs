use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::{
    launcher::{LauncherProvider, LauncherType, LoadContext},
    loader::{Loader, utils::RawLauncher},
    ui::widgets::RenderableChild,
    utils::errors::SherlockMessage,
};

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
        Loader::load_applications(
            Arc::clone(&launcher),
            &ctx.counts,
            ctx.max_decimals,
            self.use_keywords,
        )
        .map(|ad| {
            ad.into_iter()
                .map(|inner| RenderableChild::App {
                    launcher: Arc::clone(&launcher),
                    inner,
                })
                .collect()
        })
    }
}
