use std::path::PathBuf;

use crate::loader::flag_loader::parser::{ArgParser, ParsedArgs};

use super::Loader;

mod actions;
mod flags;
mod parser;
mod utils;

const DOCS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/docs/src/launchers.md");

#[derive(PartialEq)]
pub enum DebugAction {
    Help,
    Version,
    Docs,
    GenerateDocs,
    Repair,
    Init { path: PathBuf, extension: String },
}

impl Loader {
    /// This loads the application flags.
    pub fn load_flags() -> ParsedArgs {
        ArgParser::from_env()
    }
}
