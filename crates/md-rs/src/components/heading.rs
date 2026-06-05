use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use md_rs_derive::{ComponentBuilder, HeadingConstructors};

use super::{Component, span::Span};

#[derive(Default, HeadingConstructors, ComponentBuilder)]
pub struct Heading {
    level: u8,
    #[md_rs(skip_builder)]
    spans: Vec<Span>,
}
impl Heading {
    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Text(text.into()));
        self
    }

    pub fn bold(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Bold(text.into()));
        self
    }

    #[cfg(feature = "github")]
    pub fn with_text_underline(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::HtmlUnderline(text.into()));
        self
    }
}

impl Component for Heading {
    fn is_block(&self) -> bool {
        true
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        let hashes = "#".repeat(self.level as usize);
        write!(out, "{hashes} ")?;
        for span in &self.spans {
            span.render(out)?;
        }
        Ok(()) // no trailing \n
    }
}
