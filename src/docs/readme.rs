use indoc::indoc;
use md_rs::{
    components::{container::Container, heading::h1, raw::raw, span_nodes::paragraph},
    github::components::alert::{alert, warning},
    md,
};

use crate::docs::{Documentation, Installation, contributing::Contributing};

pub(super) struct Readme;
impl Documentation for Readme {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md()
            .child(raw(indoc! {r#"
                <div align="center" style="text-align:center; border-radius:10px;">
                      <picture>
                        <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.svg">
                        <source media="(prefers-color-scheme: light)" srcset="assets/logo-light.svg">
                        <img alt="sherlock logo" height="250" src="assets/logo-light.svg">
                      </picture>

                      [![Discord](https://img.shields.io/discord/1357746313646833945.svg?color=7289da&&logo=discord)](https://discord.gg/AQ44g4Yp9q)
                      <picture>
                        <img alt="application screenshot" width="100%" style="border-radius: 10px;" src="assets/mockup.png">
                      </picture>
                </div>

            "#}))
            .child(Description::docs())
            .child(h1().text("Getting Started"))
            .child(Installation::docs())
            .child(Contributing::docs())
            .child(License::docs())
    }
}

struct Description;
impl Documentation for Description {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(
            paragraph()
                .text("Sherlock is a")
                .bold("fast")
                .text(",")
                .bold("extensible")
                .text("application launcher for Wayland, build with")
                .link(
                    "GPUI",
                    "https://github.com/zed-industries/zed/tree/main/crates/gpui",
                )
                .text(".")
                .text("Sherlock's widgets inherit from launcher configurations.")
                .text("There are several launcher types, inlclugin a")
                .link("File Search", "") // TODO:
                .text(",")
                .link("Emoji Picker", "") // TODO:
                .text(", and")
                .link("Translator", "") // TODO:
                .text("."),
        )
        .child(
            alert().child(
                paragraph()
                    .text("Sherlock has been rewritten entirely, to be compatible with")
                    .code("GPUI")
                    .text("instead of")
                    .code("GTK4")
                    .text(".")
                    .text(
                        "This included major refactorings, \
                        causing some changes to configuration files.",
                    ),
            ),
        )
        .child(warning().child(
            "Disclaimer: Due to GPUI's development primarily \
            focusing on Zed, some features may not be complete yet. \
            In Sherlock, this is barely noticeable though.",
        ))
    }
}

struct License;
impl Documentation for License {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(h1().text("License")).child(
            paragraph()
                .text("GNU GENERAL LICENSE")
                .text("-")
                .text("see")
                .link(
                    "LICENSE",
                    "https://github.com/Skxxtz/sherlock/blob/main/LICENSE",
                )
                .text("for details."),
        )
    }
}
