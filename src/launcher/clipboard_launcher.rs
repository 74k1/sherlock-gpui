use serde_json::Value;

use crate::{
    launcher::{LauncherProvider, LauncherType},
    loader::utils::RawLauncher,
    ui::widgets::{RenderableChild, clipboard::ClipWidget},
    utils::{errors::SherlockMessage, intent::Capabilities},
};

/// The following arguments are available to users:
/// - `capabilities`
#[derive(Clone, Debug)]
pub struct ClipboardLauncher {
    pub capabilities: Capabilities,
}
impl LauncherProvider for ClipboardLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        let caps: Vec<String> = match raw.args.get("capabilities") {
            Some(Value::Array(arr)) => arr
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect(),
            _ => vec![String::from("calc.math"), String::from("calc.units")],
        };
        let capabilities = Capabilities::from_strings(&caps);
        LauncherType::Clipboard(ClipboardLauncher { capabilities })
    }
    fn objects(
        &self,
        launcher: std::sync::Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        _opts: std::sync::Arc<serde_json::Value>,
        _messages: &mut Vec<SherlockMessage>,
        cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        Ok(vec![RenderableChild::Clip {
            launcher,
            inner: ClipWidget::new(cx),
        }])
    }
}
