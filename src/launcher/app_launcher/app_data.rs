use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::Arc,
};

use gpui::SharedString;
use serde::{Deserialize, Serialize};

use crate::{
    launcher::{Launcher, variant_type::LauncherType},
    loader::{
        IconType, resolve_icon_path,
        utils::{ApplicationAction, ExecVariable, PriorityGuard, SherlockAlias, construct_search},
    },
    ui::launcher::context_menu::ContextMenuAction,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AppData {
    #[serde(default)]
    pub name: Option<SharedString>,
    pub exec: Option<String>,
    pub search_string: String,
    #[serde(default)]
    pub priority: PriorityGuard, // to enable new count instantly having effect
    pub icon: Option<IconType>,
    pub desktop_file: Option<PathBuf>,
    #[serde(default)]
    pub actions: Arc<[Arc<ContextMenuAction>]>,
    #[serde(default)]
    #[serde(rename = "variables")]
    pub vars: Vec<ExecVariable>,
    #[serde(default)]
    pub terminal: bool,
}
impl Eq for AppData {}
impl Hash for AppData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Make more efficient and handle error using f32
        self.exec.hash(state);
        self.desktop_file.hash(state);
    }
}
impl AppData {
    pub fn new() -> Self {
        Self {
            name: None,
            exec: None,
            search_string: String::new(),
            priority: PriorityGuard::default(),
            icon: None,
            desktop_file: None,
            actions: Arc::new([]),
            vars: vec![],
            terminal: false,
        }
    }

    pub fn apply_alias(
        &mut self,
        launcher: &Arc<Launcher>,
        alias: Option<SherlockAlias>,
        use_keywords: bool,
        mut buffer: Vec<Arc<ApplicationAction>>,
    ) {
        if let Some(alias) = alias {
            if let Some(alias_name) = alias.name.as_ref() {
                self.name = Some(SharedString::from(alias_name));
            }

            if let Some(alias_icon) = alias.icon.as_ref().map(|i| resolve_icon_path(i)) {
                self.icon = alias_icon;
            }

            let name: Option<&str> = self
                .name
                .as_ref()
                .map(|s| s.as_str())
                .or(launcher.name.as_ref().map(|s| s.as_str()));
            if let Some(alias_keywords) = alias.keywords.as_ref() {
                self.search_string = construct_search(name, alias_keywords, use_keywords);
            } else {
                self.search_string = construct_search(name, &self.search_string, use_keywords);
            }

            if let Some(alias_exec) = alias.exec.as_ref() {
                self.exec = Some(alias_exec.to_string());
            }

            if let Some(add_actions) = alias.add_actions {
                add_actions.into_iter().for_each(|mut a| {
                    if a.icon.is_none() {
                        a.icon = self.icon.clone();
                    }
                    buffer.push(a.into());
                });
            }

            if let Some(actions) = alias.actions {
                self.actions = actions
                    .into_iter()
                    .map(|mut a| {
                        if a.icon.is_none() {
                            a.icon = self.icon.clone();
                        }
                        a.into()
                    })
                    .collect();
            } else {
                self.actions = buffer
                    .into_iter()
                    .map(|a| Arc::new(ContextMenuAction::App((*a).clone())))
                    .collect::<Vec<_>>()
                    .into();
            }

            if let Some(variables) = alias.variables {
                self.vars.extend(variables);
            }
        } else {
            let name: Option<&str> = self
                .name
                .as_ref()
                .map(|s| s.as_str())
                .or(launcher.name.as_ref().map(|s| s.as_str()));
            self.search_string = construct_search(name, &self.search_string, use_keywords);
            self.actions = buffer.into_iter().map(|a| (*a).clone().into()).collect();
        }
    }
    pub fn get_exec(&self, launcher: &Arc<Launcher>) -> Option<String> {
        match &launcher.launcher_type {
            LauncherType::Web(web) => Some(format!("websearch-{}", web.engine)),

            LauncherType::Apps(_) | LauncherType::Commands(_) | LauncherType::Categories(_) => {
                self.exec.clone()
            }

            // None-Home Launchers
            LauncherType::Calculator(_) => None,
            _ => None,
        }
    }
}
