use std::time::Duration;

use gpui::{App, SharedString, Task};

use crate::utils::command_launch;

fn boottime_now() -> Duration {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) };

    Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

#[derive(Copy, Clone)]
pub(super) enum TimerState {
    Running { ends_at: Duration },
    Paused { remaining: Duration },
}
impl TimerState {
    pub(super) fn remaining(&self) -> Duration {
        match self {
            Self::Running { ends_at } => ends_at.saturating_sub(boottime_now()),
            Self::Paused { remaining } => *remaining,
        }
    }
    fn toggle(&mut self) {
        *self = match *self {
            Self::Running { ends_at } => Self::Paused {
                remaining: ends_at.saturating_sub(boottime_now()),
            },
            Self::Paused { remaining } => Self::Running {
                ends_at: boottime_now() + remaining,
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
        let ends_at = boottime_now() + duration;
        let completion_task = Self::spawn_completion_task(ends_at, &command, cx);
        Self {
            amount: duration.as_secs_f32(),
            state: TimerState::Running { ends_at },
            command,
            completion_task,
        }
    }
    /// Spawns a background task that executes `command` when `ends_at` (in CLOCK_BOOTTIME
    /// time) is reached. Unlike `async_io::Timer`, this correctly fires after the system
    /// resumes from suspend/sleep.
    ///
    /// Returns `None` if no command is configured.
    ///
    /// # Implementation
    ///
    /// Uses a Linux `timerfd` created with `CLOCK_BOOTTIME`, which the kernel guarantees
    /// will elapse even across suspend/resume cycles. The fd is registered with `async_io`
    /// so the executor can await it without polling.
    ///
    /// # Safety
    ///
    /// This function contains two `unsafe` blocks:
    ///
    /// - **`timerfd_create`**: Safe because we immediately check the return value (`fd >= 0`)
    ///   before using it, and the fd is wrapped in a `std::fs::File` which ensures it is
    ///   closed on drop.
    ///
    /// - **`FromRawFd::from_raw_fd(fd)`**: Safe because:
    ///   1. `fd` is a valid file descriptor — we asserted this above.
    ///   2. Ownership is transferred exclusively to `file`; we never use `fd` again after
    ///      this point, so there is no risk of double-close or use-after-free.
    ///   3. `std::fs::File` will close the fd when dropped, so there is no fd leak.
    fn spawn_completion_task(
        ends_at: Duration,
        command: &Option<SharedString>,
        cx: &mut App,
    ) -> Option<Task<()>> {
        command.as_ref().map(|c| {
            let cmd = c.clone();
            cx.spawn(async move |_| {
                let fd = unsafe {
                    libc::timerfd_create(
                        libc::CLOCK_BOOTTIME,
                        libc::TFD_NONBLOCK | libc::TFD_CLOEXEC,
                    )
                };
                assert!(fd >= 0, "timerfd_create failed");

                let remaining = ends_at.saturating_sub(boottime_now());
                let spec = libc::itimerspec {
                    it_interval: libc::timespec {
                        tv_sec: 0,
                        tv_nsec: 0,
                    },
                    it_value: libc::timespec {
                        tv_sec: remaining.as_secs() as i64,
                        tv_nsec: remaining.subsec_nanos() as i64,
                    },
                };
                unsafe { libc::timerfd_settime(fd, 0, &spec, std::ptr::null_mut()) };

                let file = unsafe { std::os::unix::io::FromRawFd::from_raw_fd(fd) };
                let async_file = async_io::Async::<std::fs::File>::new(file).unwrap();
                let _ = async_io::Async::readable(&async_file).await;

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
