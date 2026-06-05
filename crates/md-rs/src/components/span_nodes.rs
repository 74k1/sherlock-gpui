use std::borrow::Cow;
use std::fmt::Write;

use md_rs_derive::{ComponentConstructor, SpanNode};

use crate::components::span::LinkData;

use super::Component;
use super::span::Span;

#[derive(Default, ComponentConstructor)]
pub struct Paragraph {
    pub spans: Vec<Span>,
}
impl Paragraph {
    pub fn br(mut self) -> Self {
        self.spans.push(Span::LineBreak);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Text(text.into()));
        self
    }

    pub fn italic(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Italic(text.into()));
        self
    }

    pub fn bold(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Bold(text.into()));
        self
    }

    pub fn code(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Code(text.into()));
        self
    }

    pub fn link(
        mut self,
        title: impl Into<Cow<'static, str>>,
        target: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.spans.push(Span::Link(Box::new(LinkData {
            title: vec![Span::Text(title.into())],
            target: target.into(),
        })));
        self
    }

    pub fn link_bold(
        mut self,
        title: impl Into<Cow<'static, str>>,
        target: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.spans.push(Span::Link(Box::new(LinkData {
            title: vec![Span::Bold(title.into())],
            target: target.into(),
        })));
        self
    }

    #[cfg(feature = "github")]
    pub fn html_strong(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::HtmlStrong(text.into()));
        self
    }
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

#[derive(Default, SpanNode, ComponentConstructor)]
#[span_node(prefix = "> ")]
pub struct Blockquote {
    pub spans: Vec<Span>,
}
