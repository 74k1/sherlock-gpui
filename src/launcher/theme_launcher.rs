use std::sync::Arc;

use gpui::App;

use crate::app::theme::{ActiveTheme, ThemeData};
use crate::launcher::variant_type::InnerFunction;
use crate::launcher::{LauncherProvider, LauncherType};
use crate::loader::utils::RawLauncher;
use crate::ui::widgets::RenderableChild;
use crate::ui::widgets::theme::ThemeWidget;
use crate::utils::errors::SherlockMessage;
use crate::utils::errors::types::{DirAction, FileAction, SherlockErrorType};
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
        messages: &mut Vec<SherlockMessage>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let path_str = opts
            .get("path")
            .and_then(|p| p.as_str())
            .unwrap_or("~/.config/sherlock/themes/");
        let home = home_dir()?;
        let path = expand_path(path_str, &home);

        let builtin = [
            ("Default", ThemeData::dark()),
            ("Nord", ThemeData::nord()),
            ("Libre", ThemeData::libre()),
            ("Catppuccin Mocha", ThemeData::catppuccin_mocha()),
        ]
        .into_iter()
        .map(|(name, data)| RenderableChild::Theme {
            launcher: launcher.clone(),
            inner: ThemeWidget::new(name, Arc::new(data), true),
        });

        let custom = if path.is_dir() {
            std::fs::read_dir(&path)
                .map_err(|e| {
                    sherlock_msg!(
                        Warning,
                        SherlockErrorType::DirError(DirAction::Read, path),
                        e
                    )
                })?
                .flatten()
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("toml"))
                .filter_map(|file| {
                    let content = std::fs::read_to_string(file.path())
                        .map_err(|e| {
                            messages.push(sherlock_msg!(
                                Warning,
                                SherlockErrorType::FileError(FileAction::Read, file.path()),
                                e
                            ));
                        })
                        .ok()?;
                    let data = toml::from_str::<ThemeData>(&content)
                        .map_err(|e| {
                            messages.push(sherlock_msg!(
                                Warning,
                                SherlockErrorType::DeserializationError,
                                e
                            ))
                        })
                        .ok()?;
                    let name = file.path().file_stem()?.to_string_lossy().to_string();
                    Some(RenderableChild::Theme {
                        launcher: launcher.clone(),
                        inner: ThemeWidget::new(name, Arc::new(data), false),
                    })
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        Ok(builtin.chain(custom).collect())
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
