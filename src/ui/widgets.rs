use gpui::{AnyElement, App, AppContext, SharedString};
use std::sync::Arc;

pub mod app;
pub mod calculator;
pub mod clipboard;
pub mod dmenu;
pub mod emoji;
pub mod event;
pub mod file;
pub mod message;
pub mod mpris;
pub mod process;
pub mod script;
pub mod theme;
pub mod timer;
pub mod translator;
pub mod weather;

use crate::{
    app::theme::ThemeData,
    launcher::{
        Bind, ExecEffect, ExecMode, Launcher,
        emoji_launcher::EmojiData,
        variant_type::{InnerFunction, LauncherType, LauncherVariant},
    },
    loader::utils::{AppData, ExecVariable},
    ui::{
        launcher::context_menu::ContextMenuAction,
        widgets::{
            clipboard::ClipWidget, dmenu::DmenuData, event::EventWidget, message::MessageChild,
            mpris::MusicPlayerWidget, process::ProcessData, script::ScriptData, timer::TimerChild,
            translator::TranslationData, weather::WeatherWidget,
        },
    },
    utils::{config::HomeType, errors::SherlockMessage},
};

use calculator::CalcData;
use file::FileData;
use theme::ThemeWidget;

/// Creates enum RenderableChild,
/// ## Example:
/// ```
/// renderable_enum! {
///     enum RenderableChild {
///         App(AppData),
///         Weather(WeatherData),
///     }
/// }
/// ```
macro_rules! renderable_enum {
    (
        enum $name:ident {
            $($variant:ident($inner:ty)),* $(,)?
        }
    ) => {
        #[derive(Clone)]
        pub enum $name {
            $(
                $variant {
                    launcher: Arc<Launcher>,
                    inner: $inner,
                }
            ),*
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant { .. } => write!(f, "{}", stringify!($variant)),
                    )*
                }
            }
        }

        impl<'a> RenderableChildDelegate<'a> for $name {
            fn handles_borders(&self) -> bool {
                match self {
                    $(Self::$variant { .. } => <$inner>::HANDLES_BORDERS),*
                }
            }

            fn render(&self, selection: Selection, query: &str, theme: Arc<ThemeData>, cx: &mut App) -> AnyElement {
                match self {
                    $(Self::$variant {inner, launcher} => inner.render(launcher, selection, query, theme, cx)),*
                }
            }

            fn build_action_exec(&self, action: Arc<ContextMenuAction>) -> ExecMode {
                ExecMode::from_app_action(action, &self)
            }

            fn build_exec(&self, cx: &mut App) -> Option<ExecMode> {
                match self {
                    $(Self::$variant {launcher, inner} => {
                        inner.build_exec(launcher, cx)
                    }),*
                }
            }

            fn search(&'a self) -> &'a str {
                match self {
                    $(Self::$variant {inner, launcher} => inner.search(launcher)),*
                }
            }

            fn vars(&self, cx: &mut App) -> Option<&[ExecVariable]> {
                match self {
                    $(Self::$variant {inner, ..} => inner.vars(cx)),*
                }
            }

            fn actions(&self, cx: &mut App) -> Option<Arc<[Arc<ContextMenuAction>]>> {
                match self {
                    $(Self::$variant {inner, launcher} => inner.actions(launcher, cx)),*
                }
            }

            fn has_actions(&self, cx: &mut App) -> bool {
                match self {
                    $(Self::$variant {inner, launcher} => {
                        if launcher.actions.as_ref().map_or(false, |actions| !actions.is_empty()) {
                            return true
                        }
                        if launcher.add_actions.as_ref().map_or(false, |actions| !actions.is_empty()) {
                            return true
                        }
                        inner.has_actions(cx)
                    }),*
                }
            }

            fn binds(&self, cx: &mut App) -> Option<Arc<Vec<Bind>>> {
                match self {
                    $(Self::$variant {inner, launcher} => inner.binds(launcher, cx)),*
                }
            }

            fn execute_function(&self, func: InnerFunction, variables: &[(SharedString, SharedString)], cx: &mut App) -> Result<ExecEffect, SherlockMessage> {
                match self {
                    $(
                        Self::$variant {inner, launcher} => {
                            if let Some(first) = inner.execute_function(&func, launcher, variables, cx) {
                                return Ok(first)
                            }
                            launcher.launcher_type.execute_function(func, self, variables, cx)
                        }
                    ),*
                }
            }

            fn based_show<C: AppContext>(&self, keyword: &str, cx: &mut C) -> Option<bool> {
                match self {
                    $(Self::$variant {inner, ..} => inner.based_show(keyword, cx)),*
                }
            }

            fn sidebar(&self, cx: &mut App) -> Option<AnyElement> {
                match self {
                    $(Self::$variant {inner, ..} => inner.sidebar(cx)),*
                }
            }

            fn update_sync(&self, query: SharedString, cx: &mut App) {
                match self {
                    $(Self::$variant {inner, launcher} => inner.update_sync(query, launcher, cx)),*
                }
            }

            fn update_async<C: AppContext>(&self,  cx: &mut C) {
                match self {
                    $(Self::$variant {inner, launcher} => inner.update_async(launcher.clone(), cx)),*
                }
            }
        }

        impl<'a> LauncherValues<'a> for $name {
            fn name(&'a self) -> Option<&'a str> {
                self.launcher().name.as_ref().map(|s| s.as_str())
            }

            fn home(&self) -> HomeType {
                self.launcher().home
            }

            fn is_async(&self) -> bool {
                self.launcher().r#async
            }

            fn alias(&'a self) -> Option<&'a str> {
                self.launcher().alias.as_deref()
            }

            fn priority(&self) -> f32 {
                match self {
                    $(Self::$variant {inner, launcher} => inner.priority(launcher)),*
                }
            }

            fn spawn_focus(&self) -> bool {
                match self {
                    $(Self::$variant {launcher, ..} => launcher.spawn_focus),*
                }
            }

            fn launcher_type(&self) -> &LauncherType {
                &self.launcher().launcher_type
            }

            fn launcher_variant(&self) -> LauncherVariant {
                self.launcher().launcher_type.as_ref().into()
            }

            fn shortcut(&self) -> bool {
                match self {
                    $(Self::$variant {launcher, ..} => launcher.shortcut),*
                }
            }
        }

        impl <'a> $name {
            #[inline(always)]
            fn launcher(&'a self) -> &'a Launcher {
                match self {
                    $(Self::$variant {launcher, ..} => &launcher),*
                }
            }

            pub fn with_launcher<F, R>(&self, f: F) -> R
            where
                F: FnOnce(&Arc<Launcher>) -> R
            {
                match self {
                    $(Self::$variant { launcher, .. } => f(launcher)),*
                }
            }
        }

    };
}
renderable_enum! {
    enum RenderableChild {
        App(AppData),
        Calc(CalcData),
        Clip(ClipWidget),
        Emoji(EmojiData),
        Event(EventWidget),
        File(FileData),
        Message(MessageChild),
        Music(MusicPlayerWidget),
        Process(ProcessData),
        Script(ScriptData),
        Theme(ThemeWidget),
        Timer(TimerChild),
        Translator(TranslationData),
        Weather(WeatherWidget),
        Dmenu(DmenuData),
    }
}

impl RenderableChild {
    pub fn get_exec(&self) -> Option<String> {
        match self {
            Self::App { inner, launcher } => inner.get_exec(launcher),
            _ => None,
        }
    }
}

// To make compatible with Boxed data
#[allow(dead_code)]
pub trait HandlesBorders {
    const HANDLES_BORDERS: bool;
}

impl<T> HandlesBorders for Box<T>
where
    for<'a> T: RenderableChildImpl<'a>,
{
    const HANDLES_BORDERS: bool = <T as RenderableChildImpl<'_>>::HANDLES_BORDERS;
}

pub trait RenderableChildDelegate<'a> {
    /// Whether the child internally applies style for borders
    fn handles_borders(&self) -> bool;

    /// The logic to render the widget
    fn render(
        &self,
        selection: Selection,
        query: &str,
        theme: Arc<ThemeData>,
        cx: &mut App,
    ) -> AnyElement;

    /// Generates an execution path based on the child and the context menu action
    fn build_action_exec(&'a self, action: Arc<ContextMenuAction>) -> ExecMode;

    /// Generates an execution path when pressing return on this widget
    fn build_exec(&self, cx: &mut App) -> Option<ExecMode>;

    /// The string that contains or otherwise matces the user-provided search query
    fn search(&'a self) -> &'a str;

    /// The variable fields that should be shown next to the search input
    fn vars(&self, cx: &mut App) -> Option<&[ExecVariable]>;

    /// The context menu actions for this widget. (Gets called on the selected item only if:
    /// self.has_actions == true and the context menu gets opened)
    fn actions(&self, cx: &mut App) -> Option<Arc<[Arc<ContextMenuAction>]>>;

    /// Whether this widget owns any context menu actions. (This gets called only on the selected
    /// item)
    fn binds(&self, _cx: &mut App) -> Option<Arc<Vec<Bind>>>;

    /// Execute inner functions
    fn execute_function(
        &self,
        func: InnerFunction,
        variables: &[(SharedString, SharedString)],
        cx: &mut App,
    ) -> Result<ExecEffect, SherlockMessage>;

    /// Get inner binds for a launcher and its children
    fn has_actions(&self, cx: &mut App) -> bool;

    /// Boolean logic for conditional display (e.g., calculator)
    fn based_show<C: AppContext>(&self, keyword: &str, cx: &mut C) -> Option<bool>;

    /// Sidebar rendering
    fn sidebar(&self, cx: &mut App) -> Option<AnyElement>;

    /// Sync update on every keypress
    fn update_sync(&self, query: SharedString, cx: &mut App);

    /// Updates a dynamic renderable child that requires re-evaluation.
    ///
    /// This is used for items whose state depends on internal logic (e.g., a timer)
    /// or external factors (e.g., a weather API or file system change).
    fn update_async<C: AppContext>(&self, cx: &mut C);
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

pub trait RenderableChildImpl<'a> {
    /// If set to true, disables the inheritage of the border and background fill of the list item
    const HANDLES_BORDERS: bool = false;
    fn render(
        &self,
        launcher: &Arc<Launcher>,
        selection: Selection,
        query: &str,
        theme: Arc<ThemeData>,
        cx: &mut App,
    ) -> AnyElement;
    fn build_exec(&self, launcher: &Arc<Launcher>, cx: &mut App) -> Option<ExecMode>;
    fn priority(&self, launcher: &Arc<Launcher>) -> f32;
    fn search(&'a self, launcher: &Arc<Launcher>) -> &'a str;
    /// Will only get called once the context menu gets opened
    fn actions(
        &self,
        launcher: &Arc<Launcher>,
        _cx: &mut App,
    ) -> Option<Arc<[Arc<ContextMenuAction>]>> {
        launcher.actions.clone()
    }
    /// Whether the `additional actions` indicator should show in the status bar
    fn has_actions(&self, _cx: &mut App) -> bool {
        false
    }
    fn binds(&self, _launcher: &Arc<Launcher>, _cx: &mut App) -> Option<Arc<Vec<Bind>>> {
        None
    }
    fn execute_function(
        &self,
        _func: &InnerFunction,
        _launcher: &Arc<Launcher>,
        _variables: &[(SharedString, SharedString)],
        _cx: &mut App,
    ) -> Option<ExecEffect> {
        None
    }
    fn based_show<C: AppContext>(&self, _keyword: &str, _cx: &mut C) -> Option<bool> {
        None
    }
    fn sidebar(&self, _cx: &mut App) -> Option<AnyElement> {
        None
    }
    fn update_sync(&self, _query: SharedString, _launcher: &Arc<Launcher>, _cx: &mut App) {}
    fn update_async<C: AppContext>(&self, _launcher: Arc<Launcher>, _cx: &mut C) {}
    fn vars(&self, _cx: &mut App) -> Option<&[ExecVariable]> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Selection {
    /// The unique index of the item
    pub data_idx: usize,

    /// Whether the current item is selected by the user
    pub is_selected: bool,
}

impl Selection {
    #[inline(always)]
    pub fn new(data_idx: usize, is_selected: bool) -> Self {
        Self {
            data_idx,
            is_selected,
        }
    }
}
