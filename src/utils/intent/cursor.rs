#[derive(Clone, Copy)]
pub struct Cursor<'t> {
    tokens: &'t [&'t str],
    pos: usize,
}

#[allow(dead_code)]
impl<'t> Cursor<'t> {
    #[inline]
    pub fn new(tokens: &'t [&'t str]) -> Self {
        Self { tokens, pos: 0 }
    }
    #[inline]
    pub(super) fn peek(&self) -> Option<&'t str> {
        self.tokens.get(self.pos).copied()
    }
    #[inline]
    pub(super) fn peek2(&self) -> Option<&'t str> {
        self.tokens.get(self.pos + 1).copied()
    }
    #[inline]
    pub(super) fn advance(&mut self) -> Option<&'t str> {
        let t = self.tokens.get(self.pos).copied();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    #[inline]
    pub(super) fn remaining(&self) -> &'t [&'t str] {
        &self.tokens[self.pos..]
    }
    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    #[inline]
    pub(super) fn save(&self) -> usize {
        self.pos
    }
    #[inline]
    pub(super) fn restore(&mut self, snap: usize) {
        self.pos = snap;
    }
}

/// `matches_ignore_ascii_case!` — avoids heap allocation of `.to_lowercase()`
macro_rules! matches_ignore_ascii_case {
    ($val:expr, $($pat:literal)|+) => {{
        let v: &str = $val;
        $(v.eq_ignore_ascii_case($pat))||+
    }};
}

/// Convenience macro to match string literals case-insensitively and return
/// a `&'static str` without any allocation.
#[macro_export]
macro_rules! match_ignore_ascii {
    ($val:expr, $($pat:literal => $out:expr),* $(,)?) => {{
        let v: &str = $val;
        if false { unreachable!() }
        $(else if v.eq_ignore_ascii_case($pat) { Some($out) })*
        else { None }
    }};
}

#[inline]
pub(super) fn is_connector(t: &str) -> bool {
    matches_ignore_ascii_case!(t, "to" | "in" | "as" | "into" | "->" | "=>")
}

#[inline]
pub(super) fn is_noise(t: &str) -> bool {
    matches_ignore_ascii_case!(
        t,
        "how"
            | "much"
            | "is"
            | "are"
            | "convert"
            | "what"
            | "create"
            | "start"
            | "a"
            | "new"
            | "for"
            | "set"
            | "the"
    )
}
