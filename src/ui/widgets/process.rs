use crate::{
    app::theme::{ActiveTheme, ThemeData},
    launcher::{ExecMode, Launcher},
    loader::resolve_icon_path,
    ui::widgets::{RenderableChildImpl, Selection},
};
use gpui::{
    AnyElement, App, Image, ImageSource, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, div, img, prelude::FluentBuilder, px,
};
use procfs::ProcResult;
use std::{path::Path, sync::Arc};

#[derive(Clone, Default, Debug)]
pub struct ProcessData {
    name: SharedString,
    pid: i32,
    ppid: i32,
    icon: Option<Arc<Path>>,
}

impl ProcessData {
    pub fn new(name: SharedString, pid: i32, ppid: i32) -> Self {
        Self {
            name,
            pid,
            ppid,
            icon: None,
        }
    }

    pub fn with_icon_name(mut self, icon_name: &str) -> Self {
        self.icon = resolve_icon_path(icon_name);
        self
    }

    fn fetch_meta(&self) -> ProcResult<ProcessMeta> {
        let process = procfs::process::Process::new(self.ppid)?;
        let status = process.status()?;
        let exe: SharedString = process.exe()?.to_string_lossy().to_string().into();

        let memory_mb = status.vmrss.unwrap_or(0) as f32 / 1024.0;
        let mem_peak = status.vmhwm.unwrap_or(0) as f32 / 1024.0;
        let threads = status.threads as u32;

        let icon_name = find_desktop_icon(exe.as_str());

        Ok(ProcessMeta {
            pid: self.pid,
            ppid: self.ppid,
            memory_mb,
            mem_peak,
            threads,
            exe,
            icon: icon_name
                .as_ref()
                .and_then(|name| resolve_icon_path(name))
                .or(resolve_icon_path("sherlock-process")),
        })
    }
}

struct ProcessMeta {
    pub icon: Option<Arc<Path>>,
    pub exe: SharedString,
    pub memory_mb: f32,
    pub mem_peak: f32,
    pub threads: u32,
    pub pid: i32,
    pub ppid: i32,
}

impl<'a> RenderableChildImpl<'a> for ProcessData {
    fn render(
        &self,
        _launcher: &Arc<Launcher>,
        selection: Selection,
        theme: Arc<ThemeData>,
        _cx: &mut App,
    ) -> AnyElement {
        div()
            .px_4()
            .py_2()
            .w_full()
            .flex()
            .gap_5()
            .items_center()
            .child(if let Some(icon) = self.icon.as_ref() {
                img(Arc::clone(icon))
                    .size(px(24.))
                    .flex_shrink_0()
                    .into_any_element()
            } else {
                img(ImageSource::Image(Arc::new(Image::empty())))
                    .size(px(24.))
                    .flex_shrink_0()
                    .into_any_element()
            })
            .child(
                div()
                    .flex_col()
                    .justify_between()
                    .items_center()
                    .min_w_0()
                    .w_full()
                    .child(
                        div()
                            .font_family(theme.font_family.clone())
                            .text_sm()
                            .w_full()
                            .text_color(theme.secondary_text)
                            .when(selection.is_selected, |this| {
                                this.text_color(theme.primary_text)
                            })
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(self.name.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_family(theme.font_family.clone())
                            .text_color(theme.secondary_text)
                            .child(if self.ppid == self.pid {
                                self.pid.to_string()
                            } else {
                                format!("{} > {}", self.ppid, self.pid)
                            }),
                    ),
            )
            .into_any_element()
    }

    fn sidebar(&self, _cx: &mut App) -> Option<AnyElement> {
        let meta = self.fetch_meta().ok()?;
        Some(
            ProcessSidebar {
                meta,
                name: self.name.clone(),
            }
            .into_any_element(),
        )
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
    fn search(&'a self, _launcher: &Arc<Launcher>) -> &'a str {
        &self.name
    }
}

#[derive(IntoElement)]
struct ProcessSidebar {
    meta: ProcessMeta,
    name: SharedString,
}
impl RenderOnce for ProcessSidebar {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.global::<ActiveTheme>().0.clone();
        // Compact label/value row
        let row = |label: &'static str, value: SharedString| {
            div()
                .flex()
                .items_start()
                .justify_between()
                .gap_4()
                .py(px(4.))
                .child(
                    div()
                        .text_xs()
                        .font_family(theme.font_family.clone())
                        .text_color(theme.secondary_text)
                        .flex_shrink_0()
                        .child(label),
                )
                .child(
                    div()
                        .text_xs()
                        .font_family(theme.font_family.clone())
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .text_color(theme.primary_text)
                        .text_right()
                        .overflow_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .child(value),
                )
        };

        let section_label = |text: &'static str| {
            div()
                .text_xs()
                .font_family(theme.font_family.clone())
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme.secondary_text)
                .pt_3()
                .pb_1()
                .child(SharedString::from(text))
        };

        let separator = || div().h(px(1.)).w_full().bg(theme.border_selected).my_1();

        div()
            .min_h_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .pb(px(12.))
                    .child(
                        // Icon box
                        div()
                            .size(px(48.))
                            .rounded_lg()
                            .bg(theme.bg_muted)
                            .flex()
                            .items_center()
                            .justify_center()
                            .flex_shrink_0()
                            .child(if let Some(icon) = &self.meta.icon {
                                img(Arc::clone(icon)).size(px(32.)).into_any_element()
                            } else {
                                div().into_any_element()
                            }),
                    )
                    .child(
                        div().flex_col().gap_1().min_w_0().child(
                            div()
                                .text_sm()
                                .font_family(theme.font_family.clone())
                                .px(px(3.))
                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                .text_color(theme.primary_text)
                                .overflow_hidden()
                                .text_ellipsis()
                                .whitespace_nowrap()
                                .child(self.name.clone()),
                        ),
                    ),
            )
            .child(
                div()
                    .mt_auto()
                    .pt(px(5.))
                    .text_xs()
                    .text_color(theme.secondary_text)
                    .overflow_hidden()
                    .text_ellipsis()
                    .whitespace_nowrap()
                    .child(self.meta.exe.clone()),
            )
            .child(separator())
            .child(
                div()
                    .flex_col()
                    .child(section_label("Info"))
                    .child(row("Parent PID", format!("{}", self.meta.ppid).into()))
                    .child(row("PID", format!("{}", self.meta.pid).into())),
            )
            .child(separator())
            .child(
                div()
                    .flex_col()
                    .child(section_label("Snapshot"))
                    .child(row(
                        "Memory",
                        format!("{:.1}MB", self.meta.memory_mb).into(),
                    ))
                    .child(row(
                        "Memory Peak",
                        format!("{:.1}MB", self.meta.mem_peak).into(),
                    ))
                    .child(row("Threads", format!("{}", self.meta.threads).into())),
            )
    }
}

fn find_desktop_icon(name: &str) -> Option<String> {
    let dirs = [
        "/usr/share/applications",
        "/usr/local/share/applications",
        "~/.local/share/applications",
    ];
    for dir in dirs {
        for entry in std::fs::read_dir(dir).ok()?.flatten() {
            let content = std::fs::read_to_string(entry.path()).ok()?;
            let matches_name = content
                .lines()
                .any(|l| l.starts_with("Exec=") && l.to_lowercase().contains(&name.to_lowercase()));
            if matches_name {
                let icon = content
                    .lines()
                    .find(|l| l.starts_with("Icon="))?
                    .trim_start_matches("Icon=");
                return Some(icon.to_string());
            }
        }
    }
    None
}
