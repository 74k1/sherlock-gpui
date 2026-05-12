use std::{borrow::Cow, fmt::Write};

pub enum Span {
    Text(Cow<'static, str>),
    Bold(Cow<'static, str>),
    Italic(Cow<'static, str>),
    Code(Cow<'static, str>),
    StrikeThrough(Cow<'static, str>),
    Link {
        text: Cow<'static, str>,
        href: Cow<'static, str>,
    },
    #[cfg(feature = "github")]
    Keybind(Cow<'static, str>),
}
impl Span {
    pub fn text(text: impl Into<Cow<'static, str>>) -> Self {
        Self::Text(text.into())
    }
    pub fn bold(text: impl Into<Cow<'static, str>>) -> Self {
        Self::Bold(text.into())
    }
    pub fn italic(text: impl Into<Cow<'static, str>>) -> Self {
        Self::Italic(text.into())
    }
    pub fn code(text: impl Into<Cow<'static, str>>) -> Self {
        Self::Code(text.into())
    }
    pub fn strike_through(text: impl Into<Cow<'static, str>>) -> Self {
        Self::StrikeThrough(text.into())
    }
    #[cfg(feature = "github")]
    pub fn keybind(text: impl Into<Cow<'static, str>>) -> Self {
        Self::Keybind(text.into())
    }
    pub fn render(&self, out: &mut dyn Write) -> std::fmt::Result {
        match self {
            Span::Text(t) => write!(out, "{t}"),
            Span::Bold(t) => write!(out, "**{t}**"),
            Span::Italic(t) => write!(out, "_{t}_"),
            Span::StrikeThrough(t) => write!(out, "~~{t}~~"),
            Span::Code(t) => write!(out, "`{t}`"),
            #[cfg(feature = "github")]
            Span::Keybind(t) => write!(out, "<kbd>{t}</kbd>"),
            Span::Link { text, href } => write!(out, "[{text}]({href})"),
        }
    }
}
