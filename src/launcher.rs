pub mod app_launcher;
pub mod audio_launcher;
pub mod bookmark_launcher;
pub mod bulk_text_launcher;
pub mod calc_launcher;
pub mod category_launcher;
pub mod clipboard_launcher;
pub mod debug_launcher;
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
    launcher::variant_type::{InnerFunction, LauncherType, LauncherVariant},
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
use gpui::{App, InvalidKeystrokeError, Keystroke, SharedString};
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
    ) -> Result<ExecEffect, SherlockMessage> {
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
    callback: String,
}
impl Bind {
    pub fn matches(&self, stroke: &Keystroke) -> bool {
        &self.bind == stroke
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BindSerde {
    bind: String,
    callback: String,
    pub exit: bool,
}

impl TryFrom<BindSerde> for Bind {
    type Error = InvalidKeystrokeError;
    fn try_from(value: BindSerde) -> Result<Self, Self::Error> {
        Ok(Bind {
            bind: Keystroke::parse(&value.bind)?,
            callback: value.callback,
            exit: value.exit,
        })
    }
}
impl TryFrom<&BindSerde> for Bind {
    type Error = InvalidKeystrokeError;
    fn try_from(value: &BindSerde) -> Result<Self, Self::Error> {
        Ok(Bind {
            bind: Keystroke::parse(&value.bind)?,
            callback: value.callback.clone(),
            exit: value.exit,
        })
    }
}

#[derive(Debug, PartialEq, Default)]
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
    pub fn needs_stack_push(&self) -> bool {
        matches!(
            (&self.launcher_type).into(),
            LauncherVariant::Emoji | LauncherVariant::Process | LauncherVariant::Files
        )
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

pub enum ExecEffect {
    InsertMessages(Vec<SherlockMessage>),
    ClearMessages,
    UpdateAsync,
    None,
}

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

#[macro_export]
macro_rules! define_inner_functions {
    (
        $vis:vis enum $name:ident {
            $( $variant:ident $( { $($field_name:ident : $field_type:ty),* $(,)? } )? ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, strum::VariantNames, strum::EnumString)]
        #[strum(serialize_all = "snake_case")]
        $vis enum $name {
            $( $variant $( { $($field_name : $field_type),* } )? ),*
        }
    };
}
