use std::{sync::Arc, time::Duration};

use gpui::SharedString;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    ensure_func,
    launcher::{LauncherProvider, LauncherType, LoadContext, variant_type::InnerFunction},
    loader::utils::RawLauncher,
    sherlock_msg,
    ui::widgets::{RenderableChild, timer::TimerChild},
    utils::errors::types::SherlockErrorType,
};

#[derive(Clone, Debug, Deserialize)]
pub struct TimerLauncher {
    command: Option<SharedString>,
}

impl LauncherProvider for TimerLauncher {
    fn parse(raw: &RawLauncher) -> LauncherType {
        let command = raw
            .args
            .get("exec")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string().into());
        let launcher = TimerLauncher { command };
        LauncherType::Timer(launcher)
    }
    fn objects(
        &self,
        launcher: Arc<super::Launcher>,
        _ctx: &LoadContext,
        _opts: Arc<Value>,
        cx: &mut gpui::App,
    ) -> Result<Vec<RenderableChild>, crate::utils::errors::SherlockMessage> {
        Ok(vec![RenderableChild::Timer {
            launcher,
            inner: TimerChild::new(cx),
        }])
    }
    fn execute_function<C: gpui::AppContext>(
        &self,
        func: super::variant_type::InnerFunction,
        child: &RenderableChild,
        variables: &[(SharedString, SharedString)],
        cx: &mut C,
    ) -> Result<bool, crate::utils::errors::SherlockMessage> {
        let func = ensure_func!(func, InnerFunction::Timer);

        let RenderableChild::Timer { inner, .. } = child else {
            return Err(sherlock_msg!(
                Warning,
                SherlockErrorType::Unreachable,
                format!("Tried to unpack music tile but received: {:?}", child)
            ));
        };

        let command = match variables.first() {
            Some(v) if v.0.as_str() == "command" && !v.1.is_empty() => Some(v.1.clone()),
            _ => self.command.clone(),
        };

        match func {
            TimerLauncherFunctions::Toggle => inner.toggle(cx),
            TimerLauncherFunctions::NewTimer { duration } => inner.new_timer(duration, command, cx),
            TimerLauncherFunctions::Reset => {
                unimplemented!()
            }
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum::VariantNames, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum TimerLauncherFunctions {
    Toggle,
    Reset,
    NewTimer { duration: Duration },
}
