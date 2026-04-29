pub mod app_launcher;
pub mod audio_launcher;
pub mod bookmark_launcher;
pub mod bulk_text_launcher;
pub mod calc_launcher;
pub mod category_launcher;
pub mod clipboard_launcher;
pub mod dmenu_launcher;
pub mod emoji_launcher;
pub mod event_launcher;
pub mod file_launcher;
pub mod message_launcher;
pub mod process_launcher;
pub mod system_cmd_launcher;
pub mod theme_launcher;
pub mod timer_launcher;
pub mod translator_launcher;
pub mod utils;
pub mod variant_type;
pub mod weather_launcher;
pub mod web_launcher;
// Integrate later: TODO
// pub mod pipe_launcher;

use crate::{
    launcher::variant_type::{InnerFunction, LauncherType},
    loader::{
        LoadContext, resolve_icon_path,
        utils::{AppData, RawLauncher},
    },
    sherlock_msg,
    ui::{
        launcher::{LauncherMode, context_menu::ContextMenuAction, views::NavigationViewType},
        widgets::{
            LauncherValues, RenderableChild, RenderableChildDelegate,
            emoji::{get_emoji, get_selected_skin_tones},
        },
    },
    utils::{
        config::HomeType,
        errors::{SherlockMessage, types::SherlockErrorType},
    },
};
use gpui::{App, Keystroke, SharedString};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path, sync::Arc};

pub trait LauncherProvider {
    fn parse(raw: &RawLauncher) -> LauncherType;
    fn objects(
        &self,
        launcher: Arc<Launcher>,
        ctx: &LoadContext,
        opts: Arc<serde_json::Value>,
        messages: &mut Vec<SherlockMessage>,
        cx: &mut App,
    ) -> Result<Vec<RenderableChild>, SherlockMessage>;
    fn binds(&self) -> Option<Arc<Vec<Bind>>> {
        None
    }
    fn execute_function(
        &self,
        func: InnerFunction,
        _child: &RenderableChild,
        _variables: &[(SharedString, SharedString)],
        _cx: &mut App,
    ) -> Result<bool, SherlockMessage> {
        Err(sherlock_msg!(
            Warning,
            SherlockErrorType::InvalidFunction,
            format!("{} does not provide function: {:?}", stringify!(self), func)
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub exit: bool,
    bind: Keystroke,
    callback: InnerFunction,
}
impl Bind {
    pub fn matches(&self, stroke: &Keystroke) -> bool {
        &self.bind == stroke
    }
    pub fn get_exec(&self) -> ExecMode {
        ExecMode::Inner {
            func: self.callback.clone(),
            exit: self.exit,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BindSerde {
    bind: String,
    callback: String,
    exit: bool,
}

impl BindSerde {
    pub fn get_bind(&self, func: InnerFunction) -> Option<Bind> {
        Some(Bind {
            bind: Keystroke::parse(&self.bind).ok()?,
            callback: func,
            exit: self.exit,
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Launcher {
    /// The name of the launcher. Might get displayed in the widget
    pub name: Option<SharedString>,

    /// May not apply to all widgets
    pub icon: Option<Arc<Path>>,

    /// A short alias like `app` to launcher launcher-specific search (`alias` => only show items
    /// belonging to that launcher)
    pub alias: Option<String>,

    /// The action to be executed when the user executes a widget
    pub on_return: Option<String>,

    /// If true, Sherlock will close after the widget was exectued
    pub exit: bool,

    /// Sorting weight for display order. Lower values appear first, 0 appears only in alias mode
    pub priority: u32,

    /// If true, this item will receive async updates
    pub r#async: bool,

    /// Determines when to show the widgets
    pub home: HomeType,

    /// The category and functional variant for the launcher
    pub launcher_type: LauncherType,

    /// If true, enables UI shortcut for this widgets
    pub shortcut: bool,

    /// If true, this widget can spawn focus
    pub spawn_focus: bool,

    /// The list of primary actions. This will overwrite actions defined in possible desktop files
    pub actions: Option<Arc<[Arc<ContextMenuAction>]>>,

    /// The list of supplementary actions that extend the primary actions
    pub add_actions: Option<Arc<[Arc<ContextMenuAction>]>>,
}

impl Launcher {
    pub fn from_raw(raw: RawLauncher, launcher_type: LauncherType, icon: Option<String>) -> Self {
        Self {
            name: raw.name.map(|n| n.into()),
            icon: icon.as_deref().and_then(resolve_icon_path),
            alias: raw.alias,
            on_return: raw.on_return,
            exit: raw.exit,
            priority: raw.priority as u32,
            r#async: raw.r#async,
            home: raw.home,
            launcher_type,
            shortcut: raw.shortcut,
            spawn_focus: raw.spawn_focus,
            actions: raw.actions,
            add_actions: raw.add_actions,
        }
    }
    pub fn default_dmenu() -> Self {
        Self {
            priority: 1,
            home: HomeType::Home,
            launcher_type: LauncherType::Dmenu(dmenu_launcher::DmenuLauncher::default()),
            ..Default::default()
        }
    }
}
impl Display for Launcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name.as_ref() {
            return f.write_str(name);
        }

        f.write_str(&format!("{:?}", self.launcher_type))
    }
}

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
    CreateBookmark {
        url: String,
        name: String,
    },
    Web {
        engine: Option<String>,
        browser: Option<String>,
        exec: Option<String>,
    },
    Copy {
        content: String,
    },
    None,
}
impl ExecMode {
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
        let launcher_snapshot = data.with_launcher(|l| l.clone());

        if let Some(on_return) = launcher_snapshot.on_return.as_ref() {
            match on_return.as_str() {
                "app_launcher" | "command" => {
                    if let Some(exec) = data.get_exec() {
                        return Some(Self::Command {
                            exec: exec.to_string(),
                        });
                    }
                }
                "create_bookmark" => {
                    if let RenderableChild::App { launcher, inner } = data
                        && matches!(launcher.launcher_type, LauncherType::Clipboard(_))
                        && let (Some(exec), Some(name)) = (&inner.exec, &inner.name)
                    {
                        return Some(Self::CreateBookmark {
                            url: exec.to_string(),
                            name: name.to_string(),
                        });
                    }
                }

                k if k.starts_with("inner.") => {
                    let inner = InnerFunction::from_str(
                        data.launcher_type(),
                        k.trim_start_matches("inner."),
                    );
                    if inner != InnerFunction::Empty {
                        return Some(Self::Inner {
                            func: inner,
                            exit: launcher_snapshot.exit,
                        });
                    }
                }
                _ => {}
            };
        }

        data.build_exec(cx)
    }
    pub fn from_app_action(action: Arc<ContextMenuAction>, data: &RenderableChild) -> Self {
        match action.as_ref() {
            ContextMenuAction::App(action) => match action.method.as_str() {
                "app_launcher" | "command" => Self::Command {
                    exec: action.exec.clone().unwrap_or_default(),
                },

                "create_bookmark" => {
                    if let (Some(exec), Some(name)) = (&action.exec, &action.name) {
                        Self::CreateBookmark {
                            url: exec.to_string(),
                            name: name.to_string(),
                        }
                    } else {
                        Self::None
                    }
                }

                "web_launcher" => Self::Web {
                    engine: None,
                    browser: None,
                    exec: action.exec.clone(),
                },

                k if k.starts_with("inner.") => {
                    let inner = InnerFunction::from_str(
                        data.launcher_type(),
                        k.trim_start_matches("inner."),
                    );
                    if inner == InnerFunction::Empty {
                        Self::None
                    } else {
                        Self::Inner {
                            func: inner,
                            exit: action.exit,
                        }
                    }
                }
                _ => Self::None,
            },
            ContextMenuAction::Fn(_) => Self::DynamicContextMenuFunc { action },
            ContextMenuAction::Emoji(emj) => {
                if let Some(entry) = emj.entry() {
                    let content = get_emoji(entry, &get_selected_skin_tones())
                        .as_str()
                        .to_string();
                    Self::Copy { content }
                } else {
                    Self::None
                }
            }
        }
    }
}
