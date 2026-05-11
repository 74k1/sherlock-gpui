use std::borrow::Cow;

use super::{Component, span::Span};

pub struct Heading {
    pub level: u8,
    pub spans: Vec<Span>,
}
impl Heading {
    pub fn new(level: u8, text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            level,
            spans: vec![Span::Text(text.into())],
        }
    }
    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }
}
impl Component for Heading {
    fn render(&self, out: &mut String) {
        let hashes = "#".repeat(self.level as usize);
        out.push_str(&format!("{hashes} "));
        for span in &self.spans {
            span.render(out);
        }
        out.push('\n');
        out.push('\n');
    }
}
