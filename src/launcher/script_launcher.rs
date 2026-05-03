use gpui::{App, AppContext, SharedString};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    define_inner_functions, ensure_func,
    launcher::{
        Bind, ExecEffect, LauncherProvider, LauncherType, LoadContext, variant_type::InnerFunction,
    },
    loader::utils::RawLauncher,
    sherlock_msg, skip_func_if_nav,
    ui::{
        traits::RenderableChildImpl,
        widgets::{
            RenderableChild,
            script::{ScriptData, ScriptDataUpdateEntity},
        },
    },
    utils::errors::{SherlockMessage, types::SherlockErrorType},
};

define_inner_functions! {
    pub enum ScriptFunctions {
        Run,
    }
}

/// The following arguments are available to users:
/// - `exec`: The script to be executed
/// - `exec-args`: The arguments to the command
///
/// The following inner functions are available:
/// - `Run`: Runs the current script (if not async)
#[derive(Clone, Debug)]
pub struct ScriptLauncher {
    binds: Option<Arc<Vec<Bind>>>,
}

impl LauncherProvider for ScriptLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        let binds = raw
            .binds
            .as_ref()
            .map(|vec| Arc::new(vec.iter().filter_map(|b| Bind::try_from(b).ok()).collect()));
        LauncherType::Script(ScriptLauncher { binds })
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &LoadContext,
        opts: Arc<Value>,
        _messages: &mut Vec<SherlockMessage>,
        cx: &mut App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        let exec_command: Option<SharedString> = opts
            .get("exec")
            .and_then(|v| v.as_str())
            .map(|s| SharedString::from(s.to_owned()));

        let args: SharedString = opts
            .get("exec-args")
            .and_then(|v| v.as_str())
            .map(|s| SharedString::from(s.to_owned()))
            .unwrap_or_default();

        let Some(command) = exec_command else {
            return Err(sherlock_msg!(
                Warning,
                SherlockErrorType::ConfigError(format!(
                    "Failed to parse command from launcher configuration of launcher: {launcher}"
                )),
                format!("`exec` key is required. Received arguments: {:?}", opts)
            ));
        };

        Ok(vec![RenderableChild::Script {
            launcher,
            inner: ScriptData {
                command,
                args,
                update_entity: cx.new(|_| ScriptDataUpdateEntity::default()),
            },
        }])
    }
    fn execute_function(
        &self,
        func: super::variant_type::InnerFunction,
        child: &RenderableChild,
        _variables: &[(SharedString, SharedString)],
        cx: &mut App,
    ) -> Result<ExecEffect, SherlockMessage> {
        skip_func_if_nav!(func);
        let func = ensure_func!(func, InnerFunction::Script);
        match func {
            ScriptFunctions::Run => {
                if let RenderableChild::Script { inner, launcher } = child {
                    inner.update_async(launcher.clone(), cx);
                }
            }
        }
        Ok(ExecEffect::None)
    }
    fn binds(&self) -> Option<Arc<Vec<Bind>>> {
        self.binds.clone()
    }
}
