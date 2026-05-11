use std::sync::Arc;

use serde::Deserialize;

use crate::{
    launcher::{
        LauncherProvider,
        app_launcher::app_data::AppData,
        docs::{LauncherDoc, LauncherDocEntry},
        variant_type::LauncherType,
    },
    loader::{
        resolve_icon_path,
        utils::{PriorityGuard, RawLauncher},
    },
    ui::widgets::RenderableChild,
    utils::errors::SherlockMessage,
};

#[derive(Clone, Debug, Deserialize)]
pub struct MessageLauncher {}
impl LauncherProvider for MessageLauncher {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Message(Self {})
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        _opts: Arc<serde_json::Value>,
        _messages: &mut Vec<SherlockMessage>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let inner = AppData {
            name: Some("Show Messages".into()),
            search_string: "messages;errors;warnings;show".into(),
            icon: resolve_icon_path("sherlock-devtools"),
            priority: PriorityGuard::new_with_launcher(&launcher, 0),
            ..AppData::new()
        };
        Ok(vec![RenderableChild::App {
            launcher: Arc::clone(&launcher),
            inner,
        }])
    }
}

// DOCS
impl LauncherDoc for MessageLauncher {
    fn doc() -> LauncherDocEntry {
        LauncherDocEntry::new_hidden(
            "Messages",
            "messages",
            "The launcher to provide the message view",
        )
    }
}
