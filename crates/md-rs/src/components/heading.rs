use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use md_rs_derive::HeadingConstructors;

use super::{Component, span::Span};

#[derive(Default, HeadingConstructors)]
pub struct Heading {
    level: u8,
    spans: Vec<Span>,
}
impl Heading {
    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }
    pub fn level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }
    pub fn with_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Text(text.into()));
        self
    }
}
impl Component for Heading {
    fn render(&self, out: &mut dyn Write) -> Result {
        let hashes = "#".repeat(self.level as usize);
        write!(out, "{hashes} ")?;

        for span in &self.spans {
            span.render(out)?;
        }

        writeln!(out)?;
        writeln!(out)
    }
}
