extern crate self as md_rs;

use crate::components::container::Container;

pub mod components;
mod utils;

#[cfg(feature = "github")]
pub mod github;

pub fn md() -> Container {
    Container::default()
}
