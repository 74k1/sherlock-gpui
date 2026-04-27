use std::time::{Duration, Instant};

use gpui::{App, SharedString, Task};

use crate::utils::command_launch;

#[derive(Copy, Clone)]
pub(super) enum TimerState {
    Running { ends_at: Instant },
    Paused { remaining: Duration },
}
impl TimerState {
    pub(super) fn remaining(&self) -> Duration {
        match self {
            Self::Running { ends_at } => {
                let now = Instant::now();
                if now >= *ends_at {
                    Duration::ZERO
                } else {
                    *ends_at - now
                }
            }
            Self::Paused { remaining } => *remaining,
        }
    }
    fn toggle(&mut self) {
        *self = match *self {
            Self::Running { ends_at } => Self::Paused {
                remaining: ends_at.saturating_duration_since(Instant::now()),
            },
            Self::Paused { remaining } => Self::Running {
                ends_at: Instant::now() + remaining,
            },
        };
    }
    pub(super) fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }
}

pub(super) struct Timer {
    pub(super) amount: f32,
    pub(super) state: TimerState,
    pub(super) command: Option<SharedString>,
    pub(super) completion_task: Option<Task<()>>,
}
impl Timer {
    pub(super) fn new(duration: Duration, command: Option<SharedString>, cx: &mut App) -> Self {
        let completion_task = Self::spawn_completion_task(duration, &command, cx);
        Self {
            amount: duration.as_secs_f32(),
            state: TimerState::Running {
                ends_at: Instant::now() + duration,
            },
            command,
            completion_task,
        }
    }
    fn spawn_completion_task(
        remaining: Duration,
        command: &Option<SharedString>,
        cx: &mut App,
    ) -> Option<Task<()>> {
        command.as_ref().map(|c| {
            let cmd = c.clone();
            cx.spawn(async move |_| {
                async_io::Timer::after(remaining).await;
                let _ = command_launch::spawn_detached(&cmd, "", &[]);
            })
        })
    }
    pub(super) fn toggle(&mut self, cx: &mut App) {
        self.state.toggle();
        match self.state {
            TimerState::Paused { .. } => {
                self.completion_task.take();
            }
            TimerState::Running { .. } => {
                let remaining = self.state.remaining();
                self.completion_task = Self::spawn_completion_task(remaining, &self.command, cx);
            }
        }
    }
}
