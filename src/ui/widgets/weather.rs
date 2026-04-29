use chrono::{Local, Timelike};
use gpui::{
    Animation, AnimationExt, AnyElement, App, AppContext, Entity, FontWeight, Hsla, Image,
    ImageSource, IntoElement, ParentElement, Styled, Task, div, img, linear_gradient,
    prelude::FluentBuilder, px, relative,
};
use std::{sync::Arc, time::Duration};

use crate::{
    app::theme::ThemeData,
    launcher::{ExecMode, Launcher, variant_type::LauncherType, weather_launcher::WeatherData},
    ui::{
        utils::timeout::Timeout,
        widgets::{RenderableChildImpl, Selection},
    },
};

pub struct WeatherEntity {
    update_task: Option<Task<()>>,
}

#[derive(Clone)]
pub struct WeatherWidget {
    pub data: WeatherData,
    entity: Entity<WeatherEntity>,
}
impl WeatherWidget {
    pub fn new(data: WeatherData, cx: &mut App) -> Self {
        Self {
            data,
            entity: cx.new(|_| WeatherEntity { update_task: None }),
        }
    }
}

impl<'a> RenderableChildImpl<'a> for WeatherWidget {
    const HANDLES_BORDERS: bool = true;
    fn render(
        &self,
        launcher: &Arc<Launcher>,
        _selection: Selection,
        _query: &str,
        theme: Arc<ThemeData>,
        cx: &mut App,
    ) -> AnyElement {
        let show_datetime = if let LauncherType::Weather(wttr) = launcher.launcher_type.as_ref() {
            wttr.show_datetime
        } else {
            false
        };

        let now = Local::now();
        let time = now.time();
        if show_datetime {
            let secs_until_next_minute = 60 - now.second() as u64;
            self.entity.update(cx, |this, cx| {
                this.start_timer(Duration::from_secs(secs_until_next_minute), cx, |_, _| {});
            });
        }

        let is_init = self.data.init;
        let (p1, p2) = self
            .data
            .css
            .background(time, self.data.sunset, self.data.sunrise);
        let text_color: Hsla = self
            .data
            .css
            .color(time, self.data.sunset, self.data.sunrise)
            .into();
        div()
            .h(px(100.))
            .flex()
            .items_stretch()
            .gap(px(8.))
            // Main weather card
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .px(px(16.))
                    .py(px(12.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .rounded_xl()
                    .bg(linear_gradient(135., p1, p2))
                    .child(
                        // Left — label + condition
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.))
                            .child(
                                div()
                                    .text_color(text_color.opacity(0.6))
                                    .text_size(px(10.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .font_family(theme.font_family.clone())
                                    .child(self.data.format_str.clone()),
                            )
                            .child(
                                div()
                                    .text_color(text_color)
                                    .text_size(px(11.))
                                    .font_family(theme.font_family.clone())
                                    .child(self.data.condition.clone()),
                            ),
                    )
                    .child(
                        // Right — icon + temperature
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.))
                            .child(if let Some(icon) = self.data.icon.as_ref() {
                                img(Arc::clone(icon)).size(px(36.))
                            } else {
                                img(ImageSource::Image(Arc::new(Image::empty()))).size(px(36.))
                            })
                            .child(
                                div()
                                    .text_color(text_color)
                                    .text_size(px(40.))
                                    .line_height(relative(1.))
                                    .font_weight(FontWeight::NORMAL)
                                    .font_family(theme.font_family.clone())
                                    .child(self.data.temperature.clone()),
                            )
                            .with_animation(
                                "weather_fade_in",
                                Animation::new(Duration::from_millis(300))
                                    .with_easing(|t| t * t * (3.0 - 2.0 * t)),
                                move |this, frac| {
                                    let opacity = if is_init { frac } else { 1.0 };
                                    this.opacity(opacity.clamp(0.0, 1.0))
                                },
                            ),
                    ),
            )
            .when(show_datetime, |this| {
                this.child(
                    div()
                        .h_full()
                        .aspect_square()
                        .rounded_xl()
                        .bg(p2.color.opacity(0.85))
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_color(text_color)
                                .text_size(px(22.))
                                .line_height(relative(1.))
                                .font_weight(FontWeight::NORMAL)
                                .font_family(theme.font_family.clone())
                                .child(time.format("%H:%M").to_string()),
                        )
                        .child(
                            div()
                                .text_color(text_color.opacity(0.5))
                                .text_size(px(9.))
                                .line_height(relative(1.))
                                .font_weight(FontWeight::MEDIUM)
                                .font_family(theme.font_family.clone())
                                .child(now.format("%a %d").to_string()),
                        ),
                )
            })
            .into_any_element()
    }
    #[inline(always)]
    fn build_exec(&self, _launcher: &Arc<Launcher>, _cx: &mut App) -> Option<ExecMode> {
        None
    }
    #[inline(always)]
    fn priority(&self, launcher: &Arc<Launcher>) -> f32 {
        launcher.priority as f32
    }
    #[inline(always)]
    fn search(&self, _launcher: &Arc<Launcher>) -> &'a str {
        ""
    }
}

impl Timeout for WeatherEntity {
    fn update_task(&self) -> &Option<Task<()>> {
        &self.update_task
    }
    fn update_task_mut(&mut self) -> &mut Option<Task<()>> {
        &mut self.update_task
    }
}
