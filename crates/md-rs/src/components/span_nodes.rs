use std::borrow::Cow;

use md_rs_derive::{ComponentConstructor, SpanNode};

use super::Component;
use super::span::Span;

#[derive(Default, SpanNode, ComponentConstructor)]
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
}

#[derive(Default, SpanNode, ComponentConstructor)]
#[span_node(prefix = "> ")]
pub struct Blockquote {
    pub spans: Vec<Span>,
}
