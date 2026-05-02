use std::sync::Arc;

use gpui::{App, SharedString};

use crate::{
    launcher::{
        Launcher, LauncherValues,
        app_launcher::app_data::AppData,
        utils::binds::Bind,
        variant_type::{InnerFunction, LauncherType},
    },
    ui::{
        launcher::{LauncherMode, context_menu::ContextMenuAction, views::NavigationViewType},
        traits::RenderableChildDelegate,
        widgets::{
            RenderableChild,
            emoji::{get_emoji, get_selected_skin_tones},
        },
    },
};

#[derive(Default)]
pub enum ExecMode {
    Inner {
        func: InnerFunction,
        exit: bool,
    },
    App {
        exec: String,
        terminal: bool,
    },
    Command {
        exec: String,
    },
    Category {
        category: LauncherMode,
    },
    CreateView {
        mode: NavigationViewType,
        launcher: Arc<Launcher>,
    },
    DynamicContextMenuFunc {
        action: Arc<ContextMenuAction>,
    },
    SwitchView {
        idx: usize,
    },
    Web {
        engine: Option<String>,
        browser: Option<String>,
        exec: Option<String>,
    },
    Copy {
        content: String,
    },
    #[default]
    None,
}

impl ExecMode {
    /// Parse a method string + context into an ExecMode.
    /// This is the single source of truth for string-based dispatch.
    fn from_method(
        method: &str,
        exec: Option<impl Into<String>>,
        launcher_type: &LauncherType,
        exit: bool,
    ) -> Option<Self> {
        let exec = exec.map(Into::into);

        match method {
            "app_launcher" | "command" | "app" => Some(Self::Command {
                exec: exec.unwrap_or_default(),
            }),

            "web_launcher" | "web" | "web_search" => Some(Self::Web {
                engine: None,
                browser: None,
                exec,
            }),

            k if k.starts_with("inner.") => {
                let func = InnerFunction::from_str(launcher_type, k.trim_start_matches("inner."));
                (func != InnerFunction::Empty).then_some(Self::Inner { func, exit })
            }

            _ => None,
        }
    }

    pub fn from_bind(bind: &Bind, child: &RenderableChild) -> Option<Self> {
        Self::from_method(
            &bind.callback,
            child.get_exec(),
            child.launcher_type(),
            bind.exit,
        )
    }

    pub fn from_app_action(action: Arc<ContextMenuAction>, data: &RenderableChild) -> Self {
        match action.as_ref() {
            ContextMenuAction::App(action) => Self::from_method(
                &action.method,
                action.exec.clone(),
                data.launcher_type(),
                action.exit,
            )
            .unwrap_or_default(),

            ContextMenuAction::Fn(_) => Self::DynamicContextMenuFunc { action },
            ContextMenuAction::Emoji(emj) => emj
                .entry()
                .map(|entry| {
                    let content = get_emoji(entry, &get_selected_skin_tones())
                        .as_str()
                        .to_string();
                    Self::Copy { content }
                })
                .unwrap_or_default(),
        }
    }

    pub fn from_appdata(app_data: &AppData, launcher: &Arc<Launcher>) -> Self {
        match &launcher.launcher_type {
            LauncherType::Apps(_) => Self::App {
                exec: app_data.exec.clone().unwrap_or_default(),
                terminal: app_data.terminal,
            },
            LauncherType::Bookmarks(bkm) => Self::Web {
                engine: None,
                browser: Some(bkm.target_browser.clone()),
                exec: app_data.exec.clone(),
            },
            LauncherType::Categories(_) => Self::Category {
                category: LauncherMode::Alias {
                    short: app_data
                        .exec
                        .as_ref()
                        .map(SharedString::from)
                        .unwrap_or_default(),
                    name: app_data.name.clone().unwrap_or_default(),
                    launcher: launcher.clone(),
                },
            },
            LauncherType::Commands(_) => Self::Command {
                exec: app_data.exec.clone().unwrap_or_default(),
            },
            LauncherType::Emoji(_) => Self::CreateView {
                mode: NavigationViewType::Emoji,
                launcher: Arc::clone(launcher),
            },
            LauncherType::Files(_) => Self::CreateView {
                mode: NavigationViewType::Files { dir: None },
                launcher: Arc::clone(launcher),
            },
            LauncherType::Process(_) => Self::CreateView {
                mode: NavigationViewType::Process,
                launcher: Arc::clone(launcher),
            },
            LauncherType::Message(_) => Self::SwitchView { idx: 0 },
            LauncherType::Web(web) => Self::Web {
                engine: Some(web.engine.clone()),
                browser: web.browser.clone(),
                exec: app_data.exec.clone(),
            },
            _ => Self::None,
        }
    }
    pub fn from_child(data: &RenderableChild, cx: &mut App) -> Option<Self> {
        let launcher = data.with_launcher(|l| l.clone());
        if let Some(on_return) = &launcher.on_return
            && let Some(result) = Self::from_method(
                on_return,
                data.get_exec(),
                data.launcher_type(),
                launcher.exit,
            )
        {
            return Some(result);
        }

        data.build_exec(cx)
    }
}
