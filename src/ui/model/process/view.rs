use std::sync::Arc;

use gpui::App;

use crate::{launcher::Launcher, ui::model::Model};

pub struct ProcessView {
    pub model: Model,
}

impl ProcessView {
    pub fn new(launcher: Arc<Launcher>, cx: &mut App) -> Self {
        Self {
            model: Model::process(launcher, cx),
        }
    }
}
