use std::borrow::Cow;
use md_rs_derive::{ComponentConstructor, SpanNode, TextCompomponent};

use super::span::Span;

#[derive(Default, SpanNode, ComponentConstructor, TextCompomponent)]
pub struct Paragraph {
    pub spans: Vec<Span>,
}

impl<C: Into<Cow<'static, str>>> From<C> for Paragraph {
    fn from(value: C) -> Self {
        Self {
            spans: vec![Span::Text(value.into())],
        }
    }
}

#[derive(Default, SpanNode, ComponentConstructor, TextCompomponent)]
#[span_node(prefix = "> ")]
pub struct Blockquote {
    pub spans: Vec<Span>,
}
