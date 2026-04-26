use std::time::{Duration, Instant};

use gpui::SharedString;

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
    pub(super) fn toggle(&mut self) {
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
}
impl Timer {
    pub(super) fn new(duration: Duration, command: Option<SharedString>) -> Self {
        Self {
            amount: duration.as_secs_f32(),
            state: TimerState::Running {
                ends_at: Instant::now() + duration,
            },
            command,
        }
    }
    pub(super) fn on_completion(&self) {
        if let Some(cmd) = &self.command {
            let _ = command_launch::spawn_detached(cmd, "", &[]);
        }
    }
}
