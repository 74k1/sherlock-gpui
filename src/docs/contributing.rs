use md_rs::{
    components::{
        code_block::codeblock, container::{Container, container}, heading::h1, list::{ListItem, list}, span_nodes::paragraph
    },
    md,
};

use crate::docs::Documentation;

pub(super) struct Contributing;
impl Documentation for Contributing {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md()
            .child(h1().text("Contributing"))
            .child("Contributions are welcome! Please follow these guidelines:")
            .child(
                container().child(paragraph().bold("Prerequisites")).child(
                    paragraph()
                        .text(
                            "Ensure you have the latest stable Rust toolchain installed along with",
                        )
                        .code("rustfmt")
                        .text("and")
                        .code("clippy")
                        .text("."),
                ),
            )
            .child(
                list()
                    .title(paragraph().bold("Branching"))
                    .style("-")
                    .items([
                        ListItem::from(paragraph().code("main").text(": stable releases only")),
                        ListItem::from(
                            paragraph()
                                .text("Feature branches:")
                                .code("feat/your-feature"),
                        ),
                        ListItem::from(paragraph().text("Bug fixes:").code("fix/description")),
                    ]),
            )
            .child(
                container()
                    .child(paragraph().bold("Before opening a PR"))
                    .child(
                        codeblock()
                            .language("bash")
                            .line("cargo fmt")
                            .line("cargo clippy -- -D warnings")
                            .line("cargo test"),
                    ),
            )
            .child(
                container()
                    .child(paragraph().bold("Releasing"))
                    .child(
                        paragraph()
                            .text("Releases are automated via")
                            .code("GitHub Actions")
                            .text("on version tags:"),
                    )
                    .child(
                        codeblock()
                            .language("bash")
                            .line("git tag v0.x.0")
                            .line("git push origin v0.x.0"),
                    ),
            )
    }
}
