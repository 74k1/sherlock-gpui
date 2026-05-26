use std::sync::Arc;

use gpui::{
    AnyElement, Font, FontStyle, FontWeight, Hsla, IntoElement, ParentElement, SharedString,
    Styled, StyledText, TextRun, div,
};
use serde::{Deserialize, Serialize};

use crate::app::theme::ThemeData;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CachedPango {
    source: SharedString,
    pub text: SharedString,
    runs: Arc<[TextRun]>,
}

impl<T: Into<SharedString>> From<T> for CachedPango {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl CachedPango {
    pub fn new(source: impl Into<SharedString>) -> Self {
        Self {
            source: source.into(),
            text: SharedString::default(),
            runs: Arc::from([]),
        }
    }

    pub fn populate(&mut self, theme: &Arc<ThemeData>) {
        if self.text.is_empty() && !self.source.is_empty() {
            let (text, runs) = parse_pango(&self.source, theme);
            self.text = text.into();
            self.runs = runs.into();
        }
    }
}

impl IntoElement for CachedPango {
    type Element = StyledText;
    fn into_element(self) -> Self::Element {
        StyledText::new(self.text.clone()).with_runs(self.runs.to_vec())
    }
}

impl Serialize for CachedPango {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.source.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CachedPango {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let source = SharedString::deserialize(deserializer)?;
        Ok(Self {
            source,
            text: SharedString::default(),
            runs: Arc::from([]),
        })
    }
}

/// Minimal Pango-subset renderer: supports <b>, <i>, <br/>, HTML entities.
pub fn render_pango(
    content: &str,
    theme: &std::sync::Arc<crate::app::theme::ThemeData>,
) -> AnyElement {
    let (final_text, runs) = parse_pango(content, theme);

    div()
        .w_full()
        .overflow_hidden()
        .child(StyledText::new(SharedString::from(final_text)).with_runs(runs))
        .into_any_element()
}

fn get_attribute<'a>(tag: &'a str, attr: &str) -> Option<&'a str> {
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

/// Tokenise `content` into alternating text/tag slices and build
/// (final_text, runs).  Returns empty runs if there is no markup.
fn parse_pango(
    content: &str,
    theme: &std::sync::Arc<crate::app::theme::ThemeData>,
) -> (String, Vec<TextRun>) {
    let mut final_text = String::with_capacity(content.len());
    let mut runs: Vec<TextRun> = Vec::new();
    let mut bold_depth: usize = 0;
    let mut italic_depth: usize = 0;
    let mut span_stack: Vec<SpanState> = Vec::new();
    let mut scratch = String::new();
    let mut rest = content;

    while !rest.is_empty() {
        if let Some(tag_start) = rest.find('<') {
            if tag_start > 0 {
                scratch.clear();
                unescape_into(&rest[..tag_start], &mut scratch);
                push_run(
                    &scratch,
                    &RunContext {
                        bold_depth,
                        italic_depth,
                        family: current_family(&span_stack, theme),
                        color_override: current_color(&span_stack),
                    },
                    theme,
                    &mut final_text,
                    &mut runs,
                );
            }
            rest = &rest[tag_start..];

            if let Some(tag_end) = rest.find('>') {
                let inner = rest[1..tag_end].trim();

                if tag_eq(inner, "b") {
                    bold_depth += 1;
                } else if tag_eq(inner, "/b") {
                    bold_depth = bold_depth.saturating_sub(1);
                } else if tag_eq(inner, "i") {
                    italic_depth += 1;
                } else if tag_eq(inner, "/i") {
                    italic_depth = italic_depth.saturating_sub(1);
                } else if tag_eq(inner, "br") || tag_eq(inner, "br/") || tag_eq(inner, "br /") {
                    push_run(
                        "\n\n",
                        &RunContext {
                            bold_depth,
                            italic_depth,
                            family: current_family(&span_stack, theme),
                            color_override: current_color(&span_stack),
                        },
                        theme,
                        &mut final_text,
                        &mut runs,
                    );
                } else if tag_starts_with(inner, "span") {
                    span_stack.push(SpanState {
                        family: get_attribute(inner, "font_desc").map(SharedString::from),
                        color: get_attribute(inner, "color").and_then(parse_color),
                    });
                } else if tag_eq(inner, "/span") {
                    span_stack.pop();
                } else {
                    // unknown tag — emit literally
                    scratch.clear();
                    scratch.push('<');
                    unescape_into(&rest[1..tag_end], &mut scratch);
                    scratch.push('>');
                    push_run(
                        &scratch,
                        &RunContext {
                            bold_depth,
                            italic_depth,
                            family: current_family(&span_stack, theme),
                            color_override: current_color(&span_stack),
                        },
                        theme,
                        &mut final_text,
                        &mut runs,
                    );
                }
                rest = &rest[tag_end + 1..];
            } else {
                scratch.clear();
                unescape_into(rest, &mut scratch);
                push_run(
                    &scratch,
                    &RunContext {
                        bold_depth,
                        italic_depth,
                        family: current_family(&span_stack, theme),
                        color_override: current_color(&span_stack),
                    },
                    theme,
                    &mut final_text,
                    &mut runs,
                );
                break;
            }
        } else {
            scratch.clear();
            unescape_into(rest, &mut scratch);
            push_run(
                &scratch,
                &RunContext {
                    bold_depth,
                    italic_depth,
                    family: current_family(&span_stack, theme),
                    color_override: current_color(&span_stack),
                },
                theme,
                &mut final_text,
                &mut runs,
            );
            break;
        }
    }

    (final_text, runs)
}

struct SpanState {
    family: Option<SharedString>,
    color: Option<Hsla>,
}

fn current_family<'a>(stack: &'a [SpanState], theme: &'a ThemeData) -> &'a SharedString {
    stack
        .iter()
        .rev()
        .find_map(|s| s.family.as_ref())
        .unwrap_or(&theme.font_family)
}

fn current_color(stack: &[SpanState]) -> Option<Hsla> {
    stack.iter().rev().find_map(|s| s.color)
}

fn tag_eq(a: &str, b: &str) -> bool {
    a.len() == b.len()
        && a.bytes()
            .zip(b.bytes())
            .all(|(x, y)| x.to_ascii_lowercase() == y)
}

fn tag_starts_with(a: &str, b: &str) -> bool {
    a.len() >= b.len()
        && a[..b.len()]
            .bytes()
            .zip(b.bytes())
            .all(|(x, y)| x.to_ascii_lowercase() == y)
}

struct RunContext<'a> {
    bold_depth: usize,
    italic_depth: usize,
    family: &'a SharedString,
    color_override: Option<Hsla>,
}

fn push_run(
    text: &str,
    ctx: &RunContext,
    theme: &std::sync::Arc<crate::app::theme::ThemeData>,
    final_text: &mut String,
    runs: &mut Vec<TextRun>,
) {
    if text.is_empty() {
        return;
    }

    let start = final_text.len();
    final_text.push_str(text);
    let len = final_text.len() - start;

    let target_color = ctx.color_override.unwrap_or(if ctx.bold_depth > 0 {
        theme.primary_text
    } else {
        theme.secondary_text
    });

    let target_weight = if ctx.bold_depth > 0 {
        FontWeight::BOLD
    } else {
        FontWeight::NORMAL
    };
    let target_style = if ctx.italic_depth > 0 {
        FontStyle::Italic
    } else {
        FontStyle::Normal
    };

    // Merge adjacent run if style AND font family are identical
    if let Some(last) = runs.last_mut() {
        let same_bold = last.font.weight == target_weight;
        let same_italic = last.font.style == target_style;
        let same_family = &last.font.family == ctx.family;
        let same_color = last.color == target_color;

        if same_bold && same_italic && same_family && same_color {
            last.len += len;
            return;
        }
    }

    runs.push(TextRun {
        len,
        color: target_color,
        font: Font {
            family: ctx.family.clone(),
            weight: target_weight,
            style: target_style,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn strip_pango(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut rest = content;

    while !rest.is_empty() {
        match rest.find('<') {
            Some(0) => match rest.find('>') {
                Some(end) => {
                    let inner = rest[1..end].trim();
                    if matches!(inner, s if s.eq_ignore_ascii_case("br") || s.eq_ignore_ascii_case("br/") || s.eq_ignore_ascii_case("br /"))
                    {
                        out.push_str("\n\n");
                    }
                    rest = &rest[end + 1..];
                }
                None => {
                    unescape_into(rest, &mut out);
                    break;
                }
            },
            Some(tag_start) => {
                unescape_into(&rest[..tag_start], &mut out);
                rest = &rest[tag_start..];
            }
            None => {
                unescape_into(rest, &mut out);
                break;
            }
        }
    }
    out
}

#[inline]
fn unescape_into(s: &str, out: &mut String) {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'&' {
            out.push_str(&s[start..i]);
            let end = bytes[i..]
                .iter()
                .take(7)
                .position(|&b| b == b';')
                .map(|p| i + p)
                .unwrap_or(bytes.len().saturating_sub(1));
            match &s[i..=end] {
                "&quot;" => out.push('"'),
                "&amp;" => out.push('&'),
                "&lt;" => out.push('<'),
                "&gt;" => out.push('>'),
                "&nbsp;" => out.push(' '),
                "&apos;" => out.push('\''),
                other => out.push_str(other),
            }
            i = end + 1;
            start = i;
        } else {
            i += 1;
        }
    }
    out.push_str(&s[start..]);
}

fn parse_color(s: &str) -> Option<gpui::Hsla> {
    let s = s.trim().trim_start_matches('#');
    if s.len() == 6 {
        Some(gpui::rgb(u32::from_str_radix(s, 16).ok()?).into())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{FontStyle, FontWeight};
    use std::sync::Arc;

    fn dummy_theme() -> Arc<crate::app::theme::ThemeData> {
        Arc::new(crate::app::theme::ThemeData::dark())
    }

    fn parse(s: &str) -> (String, Vec<TextRun>) {
        parse_pango(s, &dummy_theme())
    }

    #[test]
    fn plain_text_produces_one_run() {
        let (text, runs) = parse("hello world");
        assert_eq!(text, "hello world");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].len, 11);
        assert_eq!(runs[0].font.weight, FontWeight::NORMAL);
    }

    #[test]
    fn bold_tag_sets_weight() {
        let (text, runs) = parse("<b>bold</b>");
        assert_eq!(text, "bold");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].font.weight, FontWeight::BOLD);
    }

    #[test]
    fn italic_tag_sets_style() {
        let (text, runs) = parse("<i>slanted</i>");
        assert_eq!(text, "slanted");
        assert_eq!(runs[0].font.style, FontStyle::Italic);
    }

    #[test]
    fn mixed_bold_and_italic() {
        let (text, runs) = parse("<b><i>both</i></b>");
        assert_eq!(text, "both");
        assert_eq!(runs[0].font.weight, FontWeight::BOLD);
        assert_eq!(runs[0].font.style, FontStyle::Italic);
    }

    #[test]
    fn bold_wrapping_plain_text() {
        let (text, runs) = parse("before <b>bold</b> after");
        assert_eq!(text, "before bold after");
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[0].font.weight, FontWeight::NORMAL);
        assert_eq!(runs[1].font.weight, FontWeight::BOLD);
        assert_eq!(runs[2].font.weight, FontWeight::NORMAL);
        // byte lengths
        assert_eq!(runs[0].len, 7); // "before "
        assert_eq!(runs[1].len, 4); // "bold"
        assert_eq!(runs[2].len, 6); // " after"
    }

    #[test]
    fn br_tag_inserts_newline() {
        let (text, runs) = parse("line1<br/>line2");
        assert_eq!(text, "line1\n\nline2");
        let total_run_len: usize = runs.iter().map(|r| r.len).sum();
        assert_eq!(total_run_len, text.len());

        assert_eq!(runs.len(), 1);
    }

    #[test]
    fn br_without_slash_also_works() {
        let (text, _) = parse("a<br>b");
        assert_eq!(text, "a\n\nb");
    }

    #[test]
    fn html_entities_unescaped() {
        let (text, _) = parse("a &amp; b &lt;c&gt; &quot;d&quot;");
        assert_eq!(text, "a & b <c> \"d\"");
    }

    #[test]
    fn nbsp_entity() {
        let (text, _) = parse("a&nbsp;b");
        assert_eq!(text, "a b");
    }

    #[test]
    fn empty_string() {
        let (text, runs) = parse("");
        assert_eq!(text, "");
        assert!(runs.is_empty());
    }

    #[test]
    fn unclosed_tag_treated_as_text() {
        let (text, _) = parse("hello <b world");
        assert!(text.contains("hello"));
    }

    #[test]
    fn unknown_tag_emitted_as_literal() {
        let (text, _) = parse("hello <stan>world</stan>");
        assert!(text.contains("<stan>"));
        assert!(text.contains("world"));
        assert!(text.contains("</stan>"));
    }

    #[test]
    fn nested_bold() {
        let (text, runs) = parse("<b>a<b>b</b>c</b>");
        assert_eq!(text, "abc");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].font.weight, FontWeight::BOLD);
    }

    #[test]
    fn adjacent_same_style_runs_are_merged() {
        let (text, runs) = parse("hello <!-- comment --> world");
        let total_len: usize = runs.iter().map(|r| r.len).sum();
        assert_eq!(total_len, text.len());
    }

    #[test]
    fn run_lengths_sum_to_text_length() {
        let cases = [
            "plain",
            "<b>bold</b> normal <i>italic</i>",
            "a &amp; <b>b &lt; <i>c</i></b> d",
            "<br/><br/>",
            "",
        ];
        let theme = dummy_theme();
        for case in &cases {
            let (text, runs) = parse_pango(case, &theme);
            let total: usize = runs.iter().map(|r| r.len).sum();
            assert_eq!(
                total,
                text.len(),
                "run lengths don't sum to text length for: {case:?}"
            );
        }
    }

    #[test]
    fn span_applies_font_family() {
        let (text, runs) = parse("normal <span font_desc='Courier'>monospace</span> normal");
        assert_eq!(text, "normal monospace normal");
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[1].font.family.as_ref(), "Courier");
    }

    #[test]
    fn nested_spans_restore_family() {
        let (_text, runs) =
            parse("<span font_desc='A'>outer <span font_desc='B'>inner</span> outer</span>");
        assert_eq!(runs.len(), 3);
        assert_eq!(runs[0].font.family.as_ref(), "A");
        assert_eq!(runs[1].font.family.as_ref(), "B");
        assert_eq!(runs[2].font.family.as_ref(), "A");
    }

    #[test]
    fn strip_pango_comprehensive() {
        let cases = [
            ("hello world", "hello world"),
            ("<b>bold</b>", "bold"),
            ("<i>italic</i>", "italic"),
            ("a<br/>b", "a\n\nb"),
            ("a<br>b", "a\n\nb"),
            ("a<br />b", "a\n\nb"),
            ("&amp;", "&"),
            ("&lt;", "<"),
            ("&gt;", ">"),
            ("&quot;", "\""),
            ("&apos;", "'"),
            ("&nbsp;", " "),
            ("<b>a &amp; b</b>", "a & b"),
            ("<span font_desc='monospace'>code</span>", "code"),
            ("<b><i>both</i></b>", "both"),
            ("", ""),
            ("just plain text", "just plain text"),
            (
                "<b>Name</b>: foo &amp; bar<br/>second line",
                "Name: foo & bar\n\nsecond line",
            ),
        ];

        for (input, expected) in &cases {
            assert_eq!(strip_pango(input), *expected, "failed for: {input:?}");
        }
    }
}
