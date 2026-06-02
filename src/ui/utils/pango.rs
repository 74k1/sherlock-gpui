use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled, StyledText, div};

use crate::ui::utils::pango::parse::parse_pango;

mod cache;
mod parse;
mod utils;

#[cfg(all(test, feature = "bench"))]
mod benches;
#[cfg(test)]
mod tests;

pub use cache::CachedPango;
pub use parse::strip_pango;

/// Minimal Pango-subset renderer: supports <b>, <i>, <br/>, HTML entities.
pub fn render_pango(
    content: &str,
    theme: &std::sync::Arc<crate::app::theme::ThemeData>,
) -> AnyElement {
    let (final_text, runs) = parse_pango(content, theme);

    div()
        .w_full()
        .overflow_hidden()
        .child(StyledText::new(SharedString::from(final_text)).with_runs(runs))
        .into_any_element()
}
