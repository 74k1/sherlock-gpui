use std::sync::Arc;

use gpui::App;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    ensure_func,
    launcher::{LauncherProvider, LauncherType, LoadContext, variant_type::InnerFunction},
    loader::{
        resolve_icon_path,
        utils::{AppData, RawLauncher},
    },
    sherlock_msg,
    ui::widgets::RenderableChild,
    utils::errors::{SherlockMessage, types::SherlockErrorType},
};

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;

#[derive(Debug, Clone, Copy, PartialEq, strum::VariantNames, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ProcessLauncherFunctions {
    Quit { pid: i32 },
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProcessLauncher {
    pub max_results: usize,
}

impl LauncherProvider for ProcessLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        let max_results = raw
            .args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(50);

        LauncherType::Process(ProcessLauncher { max_results })
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &LoadContext,
        opts: Arc<Value>,
        _cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        if opts
            .get("show_tile")
            .is_some_and(|s| s.as_bool().unwrap_or_default())
        {
            Ok(vec![RenderableChild::App {
                inner: AppData {
                    name: launcher.name.clone(),
                    icon: launcher
                        .icon
                        .clone()
                        .or(resolve_icon_path("sherlock-process")),
                    ..AppData::new()
                },
                launcher,
            }])
        } else {
            Ok(vec![])
        }
    }
    fn execute_function(
        &self,
        func: super::variant_type::InnerFunction,
        _child: &RenderableChild,
        _variables: &[(gpui::SharedString, gpui::SharedString)],
        _cx: &mut App,
    ) -> Result<bool, SherlockMessage> {
        let func = ensure_func!(func, InnerFunction::Process);

        match func {
            ProcessLauncherFunctions::Quit { pid } => kill_process(pid)?,
        }

        Ok(true)
    }
}

fn kill_process(pid: i32) -> Result<(), SherlockMessage> {
    let child = Pid::from_raw(pid);
    kill(child, Signal::SIGKILL).map_err(|e| sherlock_msg!(Warning, SherlockErrorType::IO, e))
}
