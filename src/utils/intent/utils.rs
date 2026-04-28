use std::fmt::Display;

use gpui::SharedString;

use crate::utils::intent::{Intent, cursor::is_noise};

impl Intent {
    pub fn tokenize(input: &str) -> impl Iterator<Item = &str> {
        input
            .split([' ', '(', ')', '%', ','])
            .map(|s| s.trim_matches(','))
            .filter(|s| !s.is_empty())
            .flat_map(|s| {
                if s.starts_with('#') {
                    return Either::Right(std::iter::once(s));
                }
                let split_pos = s
                    .char_indices()
                    .skip(1)
                    .find(|(i, c)| {
                        let prev = s[..*i].chars().last().unwrap();
                        prev.is_ascii_digit() != c.is_ascii_digit() && prev != '.' && *c != '.'
                    })
                    .map(|(i, _)| i);
                match split_pos {
                    Some(pos) => Either::Left([&s[..pos], &s[pos..]].into_iter()),
                    None => Either::Right(std::iter::once(s)),
                }
            })
    }

    pub fn tokenize_kill_noise(input: &str) -> impl Iterator<Item = &str> {
        Self::tokenize(input).filter(|w| !is_noise(w))
    }
}

#[derive(Debug, Clone)]
pub enum IntentResult {
    String(SharedString),
    Color(u32),
}
impl Display for IntentResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Color(hex) => write!(f, "#{:06x}", hex),
        }
    }
}

enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T, L: Iterator<Item = T>, R: Iterator<Item = T>> Iterator for Either<L, R> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            Either::Left(l) => l.next(),
            Either::Right(r) => r.next(),
        }
    }
}
