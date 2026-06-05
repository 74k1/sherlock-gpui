use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use md_rs_derive::{ComponentBuilder, ComponentConstructor};

use crate::components::Component;

pub enum ImageSource {
    Dark(Cow<'static, str>),
    Light(Cow<'static, str>),
    Media {
        query: Cow<'static, str>,
        src: Cow<'static, str>,
    },
}

pub fn source_dark(source: impl Into<Cow<'static, str>>) -> ImageSource {
    ImageSource::Dark(source.into())
}
pub fn source_light(source: impl Into<Cow<'static, str>>) -> ImageSource {
    ImageSource::Light(source.into())
}

#[derive(Default, ComponentConstructor, ComponentBuilder)]
pub struct Image {
    #[md_rs(skip_builder)]
    sources: Vec<ImageSource>,
    src: Option<Cow<'static, str>>,
    height: Option<Cow<'static, str>>,
    width: Option<Cow<'static, str>>,
    alt: Option<Cow<'static, str>>,
}
impl Image {
    pub fn source(mut self, source: ImageSource) -> Self {
        self.sources.push(source);
        self
    }
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl Component for Image {
    fn is_block(&self) -> bool {
        true
    }

    fn render_inline(&self, out: &mut dyn Write) -> Result {
        let Some(src) = &self.src else {
            return Ok(());
        };

        // no sources, just a plain img
        if self.sources.is_empty() {
            write!(out, "<img")?;
            if let Some(w) = &self.width {
                write!(out, r#" width="{w}""#)?;
            }
            if let Some(h) = &self.height {
                write!(out, r#" height="{h}""#)?;
            }
            if let Some(a) = &self.alt {
                write!(out, r#" alt="{}""#, escape_attr(a))?;
            }
            write!(out, r#" src="{src}">"#)?;
            return Ok(());
        }

        writeln!(out, "<picture>")?;
        for source in &self.sources {
            match source {
                ImageSource::Dark(path) => {
                    writeln!(
                        out,
                        r#"  <source media="(prefers-color-scheme: dark)" srcset="{path}">"#
                    )?;
                }
                ImageSource::Light(path) => {
                    writeln!(
                        out,
                        r#"  <source media="(prefers-color-scheme: light)" srcset="{path}">"#
                    )?;
                }
                ImageSource::Media { query, src: msrc } => {
                    writeln!(
                        out,
                        r#"  <source media="{}" srcset="{msrc}">"#,
                        escape_attr(query)
                    )?;
                }
            }
        }
        write!(out, "  <img")?;
        if let Some(w) = &self.width {
            write!(out, r#" width="{w}""#)?;
        }
        if let Some(h) = &self.height {
            write!(out, r#" height="{h}""#)?;
        }
        if let Some(a) = &self.alt {
            write!(out, r#" alt="{}""#, escape_attr(a))?;
        }
        writeln!(out, r#" src="{src}">"#)?;
        write!(out, "</picture>")
    }
}
