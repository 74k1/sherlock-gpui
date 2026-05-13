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
    HtmlStrong(Cow<'static, str>),
    #[cfg(feature = "github")]
    Keybind(Cow<'static, str>),
}
impl Span {
    pub fn render(&self, out: &mut dyn Write) -> std::fmt::Result {
        match self {
            Span::Text(t) => write!(out, "{t}"),
            Span::Bold(t) => write!(out, "**{t}**"),
            Span::Italic(t) => write!(out, "_{t}_"),
            Span::StrikeThrough(t) => write!(out, "~~{t}~~"),
            Span::Code(t) => write!(out, "`{t}`"),
            #[cfg(feature = "github")]
            Span::HtmlStrong(t) => write!(out, "<strong>{t}</strong>"),
            #[cfg(feature = "github")]
            Span::Keybind(t) => write!(out, "<kbd>{t}</kbd>"),
            Span::Link { text, href } => write!(out, "[{text}]({href})"),
        }
    }
}
