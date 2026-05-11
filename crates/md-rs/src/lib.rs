use crate::components::container::Container;

pub mod components;

/// Usage:
/// ```rust
/// md()
///     .child(Heading::new(1, "Hello World"))
///     .child(Paragraph::new().span(Bold::new("important")))
///     .child(CodeBlock::new("rust").content(src))
/// ```
pub fn md() -> Container {
    Container::default()
}
