use std::fmt::Write;
use std::{borrow::Cow, fmt::Result};

use md_rs_derive::ComponentConstructor;

use crate::components::{Component, span::Span};

#[derive(Default)]
pub enum AlertKind {
    #[default]
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl AlertKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Note => "NOTE",
            Self::Tip => "TIP",
            Self::Important => "IMPORTANT",
            Self::Warning => "WARNING",
            Self::Caution => "CAUTION",
        }
    }
}

#[derive(Default, ComponentConstructor)]
pub struct Alert {
    kind: AlertKind,
    spans: Vec<Span>,
}

impl Alert {
    pub fn kind(mut self, kind: AlertKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.spans.push(Span::Text(text.into()));
        self
    }

    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }
}

// kind impls
impl Alert {
    pub fn warning() -> Self {
        Self::default().kind(AlertKind::Warning)
    }
    pub fn note() -> Self {
        Self::default().kind(AlertKind::Note)
    }
    pub fn tip() -> Self {
        Self::default().kind(AlertKind::Tip)
    }
    pub fn important() -> Self {
        Self::default().kind(AlertKind::Important)
    }
    pub fn caution() -> Self {
        Self::default().kind(AlertKind::Caution)
    }
}

impl Component for Alert {
    fn render(&self, out: &mut dyn Write) -> Result {
        writeln!(out, "> [!{}]", self.kind.as_str())?;
        write!(out, "> ")?;
        for span in &self.spans {
            span.render(out)?;
        }
        writeln!(out)
    }
}
