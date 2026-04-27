use std::sync::Arc;

use gpui::{Styled, prelude::FluentBuilder};

use crate::{app::theme::ThemeData, ui::widgets::Selection};

pub trait ListItemBorder: Styled + FluentBuilder + Sized {
    fn list_item_border(self, theme: &Arc<ThemeData>, selection: &Selection) -> Self {
        self.bg(theme.bg_idle)
            .rounded_md()
            .border_1()
            .when(selection.is_selected, |this| {
                this.bg(theme.bg_selected)
                    .border_color(theme.border_selected)
            })
    }
}

impl<T: Styled + FluentBuilder + Sized> ListItemBorder for T {}
