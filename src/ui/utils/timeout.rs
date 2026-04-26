use std::time::Duration;

use gpui::{AsyncApp, Context, Task};

pub trait Timeout: 'static {
    const DURATION: Duration;

    fn update_task(&self) -> &Option<Task<()>>;
    fn update_task_mut(&mut self) -> &mut Option<Task<()>>;

    fn start_timer<F, R>(&mut self, cx: &mut Context<Self>, f: F)
    where
        Self: Sized,
        F: FnOnce(&mut Self, &mut Context<Self>) -> R + 'static,
    {
        if self.update_task().is_some() {
            return;
        }
        *self.update_task_mut() = Some(cx.spawn(
            |weak_self: gpui::WeakEntity<Self>, cx: &mut AsyncApp| {
                let mut cx = cx.clone();
                async move {
                    async_io::Timer::after(Self::DURATION).await;
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
