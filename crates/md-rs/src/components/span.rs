use std::{borrow::Cow, fmt::Write};

pub struct LinkData {
    pub title: Vec<Span>,
    pub target: Cow<'static, str>,
}

pub enum Span {
    LineBreak,
    Text(Cow<'static, str>),
    Bold(Cow<'static, str>),
    Italic(Cow<'static, str>),
    Code(Cow<'static, str>),
    StrikeThrough(Cow<'static, str>),
    Underline(Cow<'static, str>),
    #[cfg(feature = "github")]
    HtmlStrong(Cow<'static, str>),
    #[cfg(feature = "github")]
    HtmlUnderline(Cow<'static, str>),
    #[cfg(feature = "github")]
    Keybind(Cow<'static, str>),
    Link(Box<LinkData>),
}
impl Span {
    pub fn render(&self, out: &mut dyn Write) -> std::fmt::Result {
        match self {
            Span::LineBreak => writeln!(out, "\n"),
            Span::Text(t) => write!(out, "{t}"),
            Span::Bold(t) => write!(out, "**{t}**"),
            Span::Italic(t) => write!(out, "_{t}_"),
            Span::StrikeThrough(t) => write!(out, "~~{t}~~"),
            Span::Underline(t) => write!(out, "<u>{t}</u>"),
            Span::Code(t) => write!(out, "`{t}`"),
            #[cfg(feature = "github")]
            Span::HtmlStrong(t) => write!(out, "<strong>{t}</strong>"),
            #[cfg(feature = "github")]
            Span::HtmlUnderline(t) => write!(out, "<ins>{t}</ins>"),
            #[cfg(feature = "github")]
            Span::Keybind(t) => write!(out, "<kbd>{t}</kbd>"),
            Span::Link(link) => {
                write!(out, "[")?;
                for (i, span) in link.title.iter().enumerate() {
                    if i > 0 && span.needs_space_before() {
                        write!(out, " ")?;
                    }
                    span.render(out)?;
                }
                write!(out, "]({})", link.target)
            }
        }
    }
    #[inline(always)]
    pub fn needs_space_before(&self) -> bool {
        match self {
            Span::LineBreak => true,
            Span::Text(t) => !t.starts_with([',', '.', '!', '?', ':', ';', ')']),
            _ => true,
        }
    }
}
