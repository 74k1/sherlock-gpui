use std::path::Path;

use crate::{
    loader::flag_loader::{DebugAction, flags::FLAGS, utils::FlagSection},
    sherlock_msg,
    tokio_utils::SizedMessageObj,
    utils::{
        config::SherlockConfig,
        errors::{SherlockMessage, types::SherlockErrorType},
        networking::ClientMessage,
    },
};

#[derive(PartialEq)]
pub enum StartupAction {
    Debug(DebugAction),
    Server { msg: ClientMessage, exit: bool },
}

impl From<DebugAction> for StartupAction {
    fn from(value: DebugAction) -> Self {
        Self::Debug(value)
    }
}
impl From<ClientMessage> for StartupAction {
    fn from(value: ClientMessage) -> Self {
        Self::Server {
            msg: value,
            exit: true,
        }
    }
}

impl TryFrom<&StartupAction> for SizedMessageObj {
    type Error = SherlockMessage;
    fn try_from(value: &StartupAction) -> Result<Self, Self::Error> {
        match value {
            StartupAction::Debug(_) => Err(sherlock_msg!(
                Error,
                SherlockErrorType::Unreachable,
                "Tried to use `StartupAction::Debug` as a `SizedMessageObj`"
            )),
            StartupAction::Server { msg, .. } => SizedMessageObj::from_struct(msg),
        }
    }
}

#[allow(unused)]
impl StartupAction {
    pub fn exit(&self) -> bool {
        match self {
            Self::Server { exit, .. } => *exit,
            _ => true,
        }
    }
    pub fn with_exit(mut self, exit: bool) -> Self {
        if let Self::Server {
            exit: ref mut internal_exit,
            ..
        } = self
        {
            *internal_exit = exit;
        }
        self
    }
}

pub(super) fn init_config(path: &Path, extension: &str) {
    if let Err(e) = SherlockConfig::to_file(path, extension) {
        eprintln!("{:?}", e)
    }
}

pub(super) fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Sherlock v{}", version);
    println!("Developed by Skxxtz and Sherlock's awesome community.");
}

pub(super) fn flag_documentation() {
    let longest = FLAGS
        .iter()
        .map(|f| f.long.len() + f.short.map_or(0, |s| s.len() + 2))
        .max()
        .unwrap_or(20)
        + 4;

    let mut current_section = FlagSection::None;
    for spec in FLAGS {
        if spec.section == FlagSection::None {
            continue;
        }

        if spec.section != current_section {
            current_section = spec.section;
            println!("\n{current_section}:");
        }
        let flag_str = match spec.short {
            Some(s) => format!("{}, {}", s, spec.long),
            None => spec.long.to_string(),
        };
        println!("  {:<width$} {}", flag_str, spec.help, width = longest);
    }

    println!(
        "\n\nFor more help:\nhttps://github.com/Skxxtz/sherlock/blob/documentation/docs/flags.md\n"
    );
}
