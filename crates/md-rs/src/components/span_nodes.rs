use std::borrow::Cow;
use std::fmt::Write;

use md_rs_derive::{ComponentConstructor, SpanNode, TextCompomponent};

use super::Component;
use super::span::Span;

#[derive(Default, ComponentConstructor, TextCompomponent)]
pub struct Paragraph {
    pub spans: Vec<Span>,
}

impl Component for Paragraph {
    fn render_inline(&self, out: &mut dyn Write) -> std::fmt::Result {
        for (i, span) in self.spans.iter().enumerate() {
            if i > 0 {
                let prev = &self.spans[i - 1];
                let needs_space = span.needs_space_before() && !matches!(prev, Span::LineBreak);
                if needs_space {
                    write!(out, " ")?;
                }
            }
            span.render(out)?;
        }
        Ok(())
    }
}

impl<C: Into<Cow<'static, str>>> From<C> for Paragraph {
    fn from(value: C) -> Self {
        Self {
            spans: vec![Span::Text(value.into())],
        }
    }
}
impl From<Span> for Paragraph {
    fn from(value: Span) -> Self {
        Self { spans: vec![value] }
    }
}

#[derive(Default, SpanNode, ComponentConstructor, TextCompomponent)]
#[span_node(prefix = "> ")]
pub struct Blockquote {
    pub spans: Vec<Span>,
}
