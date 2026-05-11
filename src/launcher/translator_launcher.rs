use std::sync::Arc;

use indoc::indoc;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    launcher::{
        LauncherProvider, LauncherType, LoadContext,
        docs::{Example, LauncherDoc, LauncherDocEntry},
    },
    loader::utils::RawLauncher,
    ui::widgets::{RenderableChild, translator::TranslationData},
    utils::errors::SherlockMessage,
};

/// No user-side arguments
#[derive(Clone, Debug, Deserialize)]
pub struct Translator {}

impl LauncherProvider for Translator {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Translator(Translator {})
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &LoadContext,
        _opts: Arc<Value>,
        _messages: &mut Vec<SherlockMessage>,
        cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        Ok(vec![RenderableChild::Translator {
            launcher,
            inner: TranslationData::new(cx),
        }])
    }
}

// DOCS
impl LauncherDoc for Translator {
    fn doc() -> LauncherDocEntry {
        LauncherDocEntry {
            name: "Translator",
            variant_name: "translator",
            description: "Translate your queries into other languages.",
            args: &[],
            inner_functions: &[],
            examples: &[Example {
                description: "Basic translator",
                json: indoc! {
                    r#"{
                        "name": "Translator",
                        "alias": "trans",
                        "type": "translator",
                        "args": {},
                        "on_return": "inner.run",
                        "exit": false,
                        "priority": 1,
                        "shortcut": false
                    }"#
                },
            }],
            hidden: false,
        }
    }
}
