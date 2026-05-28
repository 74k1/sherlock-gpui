use gpui::{Hsla, SharedString};

use crate::app::theme::ThemeData;

pub(super) enum TagKind {
    BoldOpen,
    BoldClose,
    ItalicOpen,
    ItalicClose,
    Br,
    SpanOpen,
    SpanClose,
    Unknown,
}

macro_rules! tag_match {
    ($b:expr, $( $tag:literal => $kind:expr ),* $(,)?) => {
        match $b {
            $( x if x.eq_ignore_ascii_case($tag.as_bytes()) => $kind, )*
            _ => TagKind::Unknown,
        }
    };
}

pub(super) fn classify_tag(inner: &str) -> TagKind {
    let name = inner
        .trim()
        .trim_end_matches('/')
        .trim()
        .split_ascii_whitespace()
        .next()
        .unwrap_or("");

    tag_match!(name.as_bytes(),
        "b"     => TagKind::BoldOpen,
        "i"     => TagKind::ItalicOpen,
        "br"    => TagKind::Br,
        "/b"    => TagKind::BoldClose,
        "/i"    => TagKind::ItalicClose,
        "span"  => TagKind::SpanOpen,
        "/span" => TagKind::SpanClose,
    )
}

pub(super) fn get_attribute<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
    let mut rest = tag;
    while let Some(pos) = rest.find(attr) {
        rest = &rest[pos + attr.len()..];
        let rest_trimmed = rest.trim_start_matches(' ');
        if let Some(rest2) = rest_trimmed.strip_prefix("='") {
            return rest2.split('\'').next();
        } else if let Some(rest2) = rest_trimmed.strip_prefix("=\"") {
            return rest2.split('"').next();
        }
    }
    None
}

pub(super) fn parse_color(s: &str) -> Option<gpui::Hsla> {
    let s = s.trim().trim_start_matches('#');
    if s.len() == 6 {
        Some(gpui::rgb(u32::from_str_radix(s, 16).ok()?).into())
    } else {
        None
    }
}

pub(super) struct SpanState {
    family: Option<SharedString>,
    color: Option<Hsla>,
}
impl SpanState {
    pub fn new(tag: &str) -> Self {
        Self {
            family: get_attribute(tag, "font_desc").map(SharedString::from),
            color: get_attribute(tag, "color").and_then(parse_color),
        }
    }
}

pub(super) fn current_family<'a>(stack: &'a [SpanState], theme: &'a ThemeData) -> &'a SharedString {
    stack
        .iter()
        .rev()
        .find_map(|s| s.family.as_ref())
        .unwrap_or(&theme.font_family)
}

pub(super) fn current_color(stack: &[SpanState]) -> Option<Hsla> {
    stack.iter().rev().find_map(|s| s.color)
}
