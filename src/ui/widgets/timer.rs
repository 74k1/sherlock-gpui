use std::{sync::Arc, time::Duration};

use gpui::{
    AnyElement, App, AppContext, Entity, IntoElement, ParentElement, Styled, Task, div, px,
};
use smallvec::SmallVec;

use crate::{
    app::theme::ThemeData,
    launcher::{
        ExecMode, Launcher, timer_launcher::TimerLauncherFunctions, variant_type::InnerFunction,
    },
    loader::utils::ExecVariable,
    ui::{
        utils::{search::SherlockSearch, timeout::Timeout},
        widgets::{
            RenderableChildImpl, Selection,
            timer::{
                model::{Timer, TimerState},
                render::{render_new_timer_pill, render_timer},
            },
        },
    },
    utils::intent::Intent,
};

mod model;
mod render;

#[derive(Default)]
struct TimerEntity {
    intent: Option<Intent>,
    timers: SmallVec<[Timer; 4]>,
    update_task: Option<Task<()>>,
}

#[derive(Clone)]
pub struct TimerChild {
    update_entity: Entity<TimerEntity>,
    variable: [ExecVariable; 1],
}
impl TimerChild {
    pub fn new(cx: &mut App) -> Self {
        let update_entity = cx.new(|_| TimerEntity::default());
        let variable = [ExecVariable::String("command".into())];
        Self {
            update_entity,
            variable,
        }
    }
    pub fn toggle<C: AppContext>(&self, cx: &mut C) {
        self.update_entity.update(cx, |this, cx| {
            this.timers
                .iter_mut()
                .for_each(|timer| timer.state.toggle());
            cx.notify();
        });
    }
    pub fn new_timer<C: AppContext>(&self, duration: Duration, cx: &mut C) {
        self.update_entity.update(cx, |this, cx| {
            if this.timers.len() < 4 {
                this.timers.push(Timer::new(duration));
                cx.notify();
            }
        })
    }
}

impl<'a> RenderableChildImpl<'a> for TimerChild {
    fn render(
        &self,
        _launcher: &Arc<Launcher>,
        _selection: Selection,
        theme: Arc<ThemeData>,
        cx: &mut App,
    ) -> AnyElement {
        let timers: SmallVec<[(TimerState, f32); 4]> = self.update_entity.update(cx, |this, cx| {
            if this.timers.iter().any(|t| t.state.is_running()) {
                // Will cause cx.notify to run
                this.start_timer(cx, |_, _| {});
            }
            this.timers.iter().map(|t| (t.state, t.amount)).collect()
        });
        let intent = self.update_entity.read(cx).intent.clone();

        let mut capacity = timers.len();
        if intent.is_some() {
            capacity += 1;
        }

        let mut children = Vec::with_capacity(capacity);
        for (state, initial_secs) in timers {
            children.push(render_timer(state, initial_secs, &theme));
        }
        if let Some(Intent::Timer { duration }) = intent
            && capacity - 1 != 4
        {
            children.push(render_new_timer_pill(duration, &theme))
        }

        if children.is_empty() {
            return div()
                .w_full()
                .px(px(16.0))
                .py(px(14.0))
                .flex()
                .items_center()
                .justify_center()
                .font_family(theme.font_family.clone())
                .text_color(theme.secondary_text)
                .child("No timers yet")
                .into_any_element();
        }

        div()
            .w_full()
            .px(px(16.0))
            .py(px(14.0))
            .flex()
            .items_center()
            .justify_center()
            .font_family(theme.font_family.clone())
            .children(children)
            .into_any_element()
    }
    #[inline(always)]
    fn build_exec(&self, _launcher: &Arc<Launcher>, cx: &mut App) -> Option<ExecMode> {
        if let Some(Intent::Timer { duration }) = self.update_entity.read(cx).intent.clone() {
            return Some(ExecMode::Inner {
                func: InnerFunction::Timer(TimerLauncherFunctions::NewTimer { duration }),
                exit: false,
            });
        }
        Some(ExecMode::Inner {
            func: InnerFunction::Timer(TimerLauncherFunctions::Toggle),
            exit: false,
        })
    }
    #[inline(always)]
    fn priority(&self, _launcher: &Arc<Launcher>) -> f32 {
        1.0
    }
    #[inline(always)]
    fn search(&'a self, _launcher: &Arc<Launcher>) -> &'a str {
        ""
    }
    #[inline(always)]
    fn based_show<C: AppContext>(&self, keyword: &str, cx: &mut C) -> Option<bool> {
        let mut tokens = Intent::tokenize_kill_noise(keyword).peekable();
        let intent = Intent::try_parse_timer(&mut tokens);
        let show = matches!(&intent, Some(Intent::Timer { .. }));

        self.update_entity.update(cx, |this, _| {
            this.intent = intent;
        });

        if show {
            return Some(true);
        }

        if keyword.fuzzy_match("timer") {
            return Some(true);
        }

        Some(false)
    }
    #[inline(always)]
    fn vars(&self, cx: &mut App) -> Option<&[crate::loader::utils::ExecVariable]> {
        if let Some(Intent::Timer { .. }) = &self.update_entity.read(cx).intent {
            return Some(&self.variable);
        }
        None
    }
}

impl Timeout for TimerEntity {
    const DURATION: Duration = Duration::from_secs(1);
    fn update_task(&self) -> &Option<Task<()>> {
        &self.update_task
    }
    fn update_task_mut(&mut self) -> &mut Option<Task<()>> {
        &mut self.update_task
    }
}
