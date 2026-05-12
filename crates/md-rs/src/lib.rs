use crate::components::container::Container;

pub mod components;

#[cfg(feature = "github")]
pub mod github;

pub fn md() -> Container {
    Container::default()
}
