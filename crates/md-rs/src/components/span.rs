use std::borrow::Cow;

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
    pub(super) fn render(&self, out: &mut String) {
        match self {
            Span::Text(t) => out.push_str(t),
            Span::Bold(t) => out.push_str(&format!("**{t}**")),
            Span::Italic(t) => out.push_str(&format!("_{t}_")),
            Span::StrikeThrough(t) => out.push_str(&format!("~~{t}~~")),
            Span::Code(t) => out.push_str(&format!("`{t}`")),
            Span::Link { text, href } => out.push_str(&format!("[{text}]({href})")),
        }
    }
}
