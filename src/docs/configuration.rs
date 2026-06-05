use md_rs::{
    components::{
        ParentComponentExt, TextComponentExt,
        container::Container,
        heading::h2,
        list::{ListItem, list},
        span_nodes::paragraph,
    },
    md,
};

use crate::{
    docs::book::{BookEntry, TopLevelEntry},
    launcher::Launcher,
    loader::utils::ExecVariable,
};

pub struct Configuration;
impl TopLevelEntry for Configuration {
    type Summary = Container;
    fn summary() -> Self::Summary {
        md().child(h2().with_text_underline("Configuration"))
            .child(
                paragraph()
                    .text("Sherlock can be customized in many ways.")
                    .br()
                    .text("Choose one topic to get started:"),
            )
            .child(list().style("-").items(Self::children().map(|child| {
                ListItem::from(paragraph().link(child.title, child.file.unwrap_or("#")))
            })))
    }
    fn children() -> impl Iterator<Item = BookEntry> + 'static {
        [
            BookEntry::of::<Launcher>()
                .with_title("Launchers")
                .with_file("launchers.md"),
            BookEntry::of::<ExecVariable>()
                .with_title("Exec Variables")
                .with_file("exec-variables.md"),
        ]
        .into_iter()
    }
}

impl From<Configuration> for BookEntry {
    fn from(_: Configuration) -> Self {
        Self {
            title: "Configuration",
            file: Some("configuration.md"),
            render_fn: Some(Configuration::summary_md),
            children: Configuration::children().collect(),
        }
    }
}
