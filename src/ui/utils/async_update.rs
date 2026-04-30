use std::sync::Arc;

use gpui::{App, AsyncApp, Entity, Task, WeakEntity};

use crate::launcher::Launcher;

pub trait Fetchable: Sized + Send + 'static {
    type Error: Send;
    fn fetch(launcher: &Arc<Launcher>) -> impl Future<Output = Result<Self, Self::Error>> + Send;
}

pub trait AsyncUpdate {
    fn update_async(&self, launcher: Arc<Launcher>, cx: &mut App);
}

pub struct AsyncUpdateEntity<T: Fetchable> {
    task: Option<Task<()>>,
    data: T,
}

impl<T: Fetchable> AsyncUpdate for Entity<AsyncUpdateEntity<T>> {
    fn update_async(&self, launcher: Arc<Launcher>, cx: &mut App) {
        self.update(cx, |this, cx| {
            // reset task
            this.task = None;
            this.task = Some(cx.spawn(
                |weak_self: WeakEntity<AsyncUpdateEntity<T>>, cx: &mut AsyncApp| {
                    let mut cx = cx.clone();
                    async move {
                        if let Ok(result) = T::fetch(&launcher).await {
                            let _ = weak_self.update(&mut cx, |this, cx| {
                                this.data = result;
                                cx.notify();
                            });
                        }
                    }
                },
            ));
        });
    }
}


