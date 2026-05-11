use std::{collections::HashSet, sync::Arc};

use gpui::AsyncApp;

use super::{SherlockConfig, watcher::ConfigFileChange};
use crate::{
    CONFIG, app::RenderableChildEntity, loader::Loader, ui::launcher::LauncherMode,
    utils::errors::SherlockMessage,
};

pub fn reload(
    cx: &AsyncApp,
    data: &RenderableChildEntity,
    initial_messages: &mut Vec<SherlockMessage>,
    changes: HashSet<ConfigFileChange>,
) -> Option<Arc<[LauncherMode]>> {
    let needs = ReloadNeeds::from_changes(&changes);
    let mut messages: Vec<SherlockMessage> = Vec::new();

    if needs.config {
        let mut flags = Loader::load_flags()?;
        let config = match flags.get_config() {
            Err(e) => {
                messages.push(e);
                let mut cfg = SherlockConfig::default();
                cfg.apply_flags(&mut flags);
                cfg
            }
            Ok((cfg, msgs)) => {
                messages.extend(msgs);
                cfg
            }
        };
        // Update global config
        if let Ok(mut guard) = CONFIG.get()?.write() {
            *guard = config;
        }
    }

    // Reload launchers
    let modes = if needs.launchers || needs.apps {
        let result = match cx.update(|cx| Loader::load_launchers(cx, data.clone())) {
            Ok(result) => result,
            Err(e) => {
                messages.push(e);
                return None;
            }
        };
        messages.extend(result.messages);
        Some(result.modes)
    } else {
        None // caller keeps existing modes
    };

    *initial_messages = messages;
    modes
}

#[derive(Default)]
struct ReloadNeeds {
    config: bool,
    launchers: bool,
    apps: bool,
}

impl ReloadNeeds {
    fn from_changes(changes: &HashSet<ConfigFileChange>) -> Self {
        changes.iter().fold(Self::default(), |mut needs, change| {
            match change {
                ConfigFileChange::Config => needs.config = true,
                ConfigFileChange::Fallback
                | ConfigFileChange::Alias
                | ConfigFileChange::Actions
                | ConfigFileChange::Ignore => needs.launchers = true,
                ConfigFileChange::Apps => needs.apps = true,
                ConfigFileChange::Other => {}
            }
            needs
        })
    }
}
