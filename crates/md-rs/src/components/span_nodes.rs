use std::borrow::Cow;
use std::fmt::Write;

use md_rs_derive::{ComponentConstructor, SpanNode};

use super::Component;
use super::span::Span;

#[derive(Default, ComponentConstructor)]
pub struct Paragraph {
    pub spans: Vec<Span>,
}
impl Paragraph {
    pub fn with_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Text(text.into()));
        self
    }
    pub fn with_text_italic(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Italic(text.into()));
        self
    }
    pub fn with_text_bold(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Bold(text.into()));
        self
    }
    pub fn with_text_code(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Code(text.into()));
        self
    }
    #[cfg(feature = "github")]
    pub fn with_html_text_string(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::HtmlStrong(text.into()));
        self
    }
}

impl Component for Paragraph {
    fn render(&self, out: &mut dyn Write) -> std::fmt::Result {
        for (i, span) in self.spans.iter().enumerate() {
            let needs_space_before = i > 0 && !matches!(span, Span::Text(_));
            let needs_space_after = i < self.spans.len() - 1 && !matches!(span, Span::Text(_));

            if needs_space_before {
                write!(out, " ")?;
            }
            span.render(out)?;
            if needs_space_after {
                write!(out, " ")?;
            }
        }
        write!(out, "\n\n")
    }
}

#[derive(Default, SpanNode, ComponentConstructor)]
#[span_node(prefix = "> ")]
pub struct Blockquote {
    pub spans: Vec<Span>,
}
