use crate::app::RenderableChildWeak;
use crate::launcher::process_launcher::ProcessLauncher;
use crate::launcher::{Launcher, variant_type::LauncherType};
use crate::ui::launcher::LauncherView;
use crate::ui::model::process::backends::ProcessBackend;
use crate::ui::widgets::RenderableChild;
use crate::ui::widgets::process::ProcessData;
use gpui::{App, Task, WeakEntity};
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc;
use utils::{ProcessResult, ResultHeap};

mod backends;
mod utils;
pub mod view;

#[derive(Default)]
pub struct ProcessModel {
    backend: ProcessBackend,
    launcher: Arc<Launcher>,
    results: Vec<ProcessResult>,
    cancel_tx: Option<mpsc::Sender<()>>,
    _poll_task: Option<Task<()>>,
}

impl ProcessModel {
    pub fn new(launcher: Arc<Launcher>) -> Self {
        if let LauncherType::Process(ProcessLauncher { max_results }) = launcher.launcher_type {
            Self {
                launcher,
                results: Vec::with_capacity(max_results),
                ..Default::default()
            }
        } else {
            Self {
                launcher,
                results: Vec::with_capacity(0),
                ..Default::default()
            }
        }
    }

    pub fn search(
        &mut self,
        query_lower: Arc<str>,
        result_entity: RenderableChildWeak,
        launcher_weak: WeakEntity<LauncherView>,
        cx: &mut App,
    ) {
        self.cancel_tx = None;
        self._poll_task = None;
        self.results.clear();

        let (cancel_tx, cancel_rx) = mpsc::channel::<()>(1);
        self.cancel_tx = Some(cancel_tx);

        let backend = self.backend.clone();
        let cap = self.results.capacity();
        let launcher = Arc::clone(&self.launcher);

        let poll_task = cx.spawn(async move |cx| {
            let results = cx
                .background_executor()
                .spawn({
                    let query = query_lower.clone();
                    async move {
                        let mut heap = ResultHeap::new(cap);
                        backend.search(query, &mut heap, cancel_rx);
                        heap.snapshot()
                    }
                })
                .await;

            let count = results.len();
            let children = Rc::new(
                results
                    .into_iter()
                    .map(|r| RenderableChild::Process {
                        launcher: Arc::clone(&launcher),
                        inner: ProcessData::new(r.name.clone(), r.pid, r.ppid)
                            .with_icon_name("sherlock-process"),
                    })
                    .collect::<Vec<_>>(),
            );
            let indices: Arc<[usize]> = (0..count).collect::<Vec<_>>().into();
            if let Some(view) = launcher_weak.upgrade() {
                cx.update(|cx| {
                    view.update(cx, |this, cx| {
                        if let Some(entity) = result_entity.upgrade() {
                            entity.update(cx, |e, _| *e = children);
                        }
                        this.apply_results(indices, query_lower, false, cx);
                    });
                });
            }
        });

        self._poll_task = Some(poll_task);
    }
}
