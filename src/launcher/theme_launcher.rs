use std::sync::Arc;

use gpui::App;

use crate::app::theme::{ActiveTheme, ThemeData};
use crate::launcher::variant_type::InnerFunction;
use crate::launcher::{LauncherProvider, LauncherType};
use crate::loader::utils::RawLauncher;
use crate::ui::widgets::RenderableChild;
use crate::ui::widgets::theme::ThemeWidget;
use crate::utils::errors::types::SherlockErrorType;
use crate::utils::files::{expand_path, home_dir};
use crate::{ensure_func, sherlock_msg};

#[derive(Debug, Clone, PartialEq, strum::VariantNames, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ThemePickerFunctions {
    Pick { theme: Arc<ThemeData> },
}

#[derive(Clone, Debug)]
pub struct ThemePicker {}

impl LauncherProvider for ThemePicker {
    fn parse(_raw: &RawLauncher) -> LauncherType {
        LauncherType::Theme(ThemePicker {})
    }
    fn objects(
        &self,
        launcher: std::sync::Arc<super::Launcher>,
        _ctx: &crate::loader::LoadContext,
        opts: std::sync::Arc<serde_json::Value>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let path_str = opts
            .get("path")
            .and_then(|p| p.as_str())
            .unwrap_or("~/.config/sherlock/themes/");

        // expand homedir
        let home = home_dir()?;
        let path = expand_path(path_str, &home);

        if path.exists() && path.is_dir() {
            Ok([
                ("Default", ThemeData::dark()),
                ("Nord", ThemeData::nord()),
                ("Libre", ThemeData::libre()),
                ("Catppuccin Mocha", ThemeData::catppuccin_mocha()),
            ]
            .into_iter()
            .map(|(name, data)| RenderableChild::Theme {
                launcher: launcher.clone(),
                inner: ThemeWidget::new(name, Arc::new(data), true),
            })
            .collect())
        } else {
            // default sherlock themes
            Ok([
                ("Default", ThemeData::dark()),
                ("Nord", ThemeData::nord()),
                ("Libre", ThemeData::libre()),
                ("Catppuccin Mocha", ThemeData::catppuccin_mocha()),
            ]
            .into_iter()
            .map(|(name, data)| RenderableChild::Theme {
                launcher: launcher.clone(),
                inner: ThemeWidget::new(name, Arc::new(data), true),
            })
            .collect())
        }
    }
    fn execute_function(
        &self,
        func: super::variant_type::InnerFunction,
        _child: &RenderableChild,
        _variables: &[(gpui::SharedString, gpui::SharedString)],
        cx: &mut App,
    ) -> Result<bool, crate::utils::errors::SherlockMessage> {
        let func = ensure_func!(func, InnerFunction::Theme);

        match func {
            ThemePickerFunctions::Pick { theme } => {
                cx.set_global(ActiveTheme(theme));
            }
        }

        Ok(true)
    }
}
