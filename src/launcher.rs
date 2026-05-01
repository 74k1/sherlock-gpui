pub mod app_launcher;
pub mod audio_launcher;
pub mod bookmark_launcher;
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
pub mod script_launcher;
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
    launcher::{
        utils::binds::Bind,
        variant_type::{InnerFunction, LauncherType, LauncherVariant},
    },
    loader::{LoadContext, resolve_icon_path, utils::RawLauncher},
    sherlock_msg,
    ui::{launcher::context_menu::ContextMenuAction, widgets::RenderableChild},
    utils::{
        config::HomeType,
        errors::{SherlockMessage, types::SherlockErrorType},
    },
};
use gpui::{App, SharedString};
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

#[allow(dead_code)]
pub trait LauncherValues<'a> {
    fn name(&'a self) -> Option<&'a str>;
    fn alias(&'a self) -> Option<&'a str>;
    fn priority(&self) -> f32;
    fn is_async(&self) -> bool;
    fn home(&self) -> HomeType;
    fn spawn_focus(&self) -> bool;
    fn launcher_type(&'a self) -> &'a LauncherType;
    fn launcher_variant(&'a self) -> LauncherVariant;
    fn shortcut(&self) -> bool;
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
