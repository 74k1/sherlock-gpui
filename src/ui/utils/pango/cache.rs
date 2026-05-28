use std::sync::Arc;

use gpui::{IntoElement, RenderOnce, SharedString, StyledText, TextRun};
use serde::{Deserialize, Serialize};

use crate::{app::theme::ThemeData, ui::utils::pango::parse_pango};

/// Wapper to enfore `CachedPango` to be populated using the current theme
#[derive(IntoElement)]
pub struct PopulatedPango(CachedPango);

impl RenderOnce for PopulatedPango {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        StyledText::new(self.0.text).with_runs(self.0.runs.to_vec())
    }
}

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

    pub fn prepared(mut self, theme: &Arc<ThemeData>) -> PopulatedPango {
        self.populate(theme);
        PopulatedPango(self)
    }

    pub fn populate(&mut self, theme: &Arc<ThemeData>) {
        if self.text.is_empty() && !self.source.is_empty() {
            let (text, runs) = parse_pango(&self.source, theme);
            self.text = text.into();
            self.runs = runs.into();
        }
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
