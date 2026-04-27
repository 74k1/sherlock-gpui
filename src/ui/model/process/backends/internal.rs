use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
};

use procfs::process::all_processes;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::ui::model::{
    process::{
        backends::ProcessSearchProvider,
        utils::{PathSearchUtils, ProcessResult, ResultHeap},
    },
    utils::CiUtils,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct InternalBackend;

impl ProcessSearchProvider for InternalBackend {
    fn name(&self) -> &'static str {
        "internal"
    }
    fn search(
        &self,
        query: Arc<str>,
        heap: &mut ResultHeap,
        mut cancel_rx: mpsc::Receiver<()>,
    ) -> bool {
        let Ok(all_procs) = all_processes() else {
            return true;
        };
        let procs: Vec<(i32, Option<String>, procfs::process::Stat)> = all_procs
            .par_bridge()
            .filter_map(Result::ok)
            .filter(|p| p.uid().is_ok_and(|uid| uid > 0))
            .filter_map(|p| {
                let stat = p.stat().ok()?;
                let name = p
                    .exe()
                    .ok()
                    .and_then(|path| path.file_name()?.to_str().map(str::to_string));
                Some((p.pid, name, stat))
            })
            .collect();
        let mut process_names: HashMap<i32, String> = procs
            .iter()
            .filter_map(|(pid, name, _)| name.clone().map(|n| (*pid, n)))
            .collect();
        let mut tmp: HashMap<i32, i32> = HashMap::new();
        for (pid, _, stat) in procs.into_iter().rev() {
            if cancel_rx.try_recv().is_ok() || cancel_rx.is_closed() {
                return false;
            }
            let entry = if stat.ppid == 1 {
                let named_id = tmp.get(&pid).copied().unwrap_or(pid);
                process_names
                    .remove(&named_id)
                    .map(|name| ((pid, named_id), name))
            } else if stat.tty_nr != 0 {
                if let Some(r) = tmp.remove(&pid) {
                    tmp.insert(stat.ppid, r);
                } else {
                    tmp.entry(stat.ppid).or_insert(pid);
                }
                None
            } else if let Entry::Vacant(e) = tmp.entry(stat.ppid) {
                e.insert(pid);
                None
            } else {
                None
            };
            let Some(((ppid, pid), name)) = entry else {
                continue;
            };
            let name_bytes = name.as_bytes();
            if !CiUtils::bytes_contain_ci(name_bytes, query.as_bytes()) {
                continue;
            }
            let score = PathSearchUtils::score_ci(name_bytes, &query);
            heap.push(ProcessResult {
                name: name.into(),
                pid,
                ppid,
                score,
            });
        }
        true
    }
}
