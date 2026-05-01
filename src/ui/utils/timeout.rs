use std::time::Duration;

use gpui::{App, WeakEntity};
use gpui::{AppContext, AsyncApp, Context, Entity, Task};

#[derive(Clone)]
pub struct TimeoutCaller<T: 'static> {
    inner: Entity<TimeoutInner<T>>,
}

pub struct TimeoutInner<T> {
    task: Option<Task<()>>,
    data: T,
}

impl<T: 'static> TimeoutCaller<T> {
    pub fn new(data: T, cx: &mut impl AppContext) -> Self {
        Self {
            inner: cx.new(|_| TimeoutInner { task: None, data }),
        }
    }

    pub fn start<F, R>(&self, duration: Duration, cx: &mut App, f: F)
    where
        F: FnOnce(&mut T, &mut Context<TimeoutInner<T>>) -> R + 'static,
    {
        self.inner.update(cx, |this, cx| {
            this.task = Some(cx.spawn(
                move |weak_self: WeakEntity<TimeoutInner<T>>, cx: &mut AsyncApp| {
                    let mut cx = cx.clone();
                    async move {
                        async_io::Timer::after(duration).await;
                        let _ = weak_self.update(&mut cx, |this, cx| {
                            f(&mut this.data, cx);
                            cx.notify();
                        });
                    }
                },
            ));
        });
    }
}

pub trait Timeout: 'static {
    fn update_task(&self) -> &Option<Task<()>>;
    fn update_task_mut(&mut self) -> &mut Option<Task<()>>;

    fn start_timer<F, R>(&mut self, duration: Duration, cx: &mut Context<Self>, f: F)
    where
        Self: Sized,
        F: FnOnce(&mut Self, &mut Context<Self>) -> R + 'static,
    {
        if self.update_task().is_some() {
            return;
        }
        *self.update_task_mut() = Some(cx.spawn(
            move |weak_self: gpui::WeakEntity<Self>, cx: &mut AsyncApp| {
                let mut cx = cx.clone();
                async move {
                    async_io::Timer::after(duration).await;
                    let _ = weak_self.update(&mut cx, |this, cx| {
                        f(this, cx);
                        cx.notify();
                        *this.update_task_mut() = None;
                    });
                }
            },
        ));
    }
}
