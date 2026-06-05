use std::borrow::Cow;

use crate::components::Component;

pub struct Raw(pub Cow<'static, str>);
impl Component for Raw {
    fn render(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.render_inline(out)
    }
    fn render_inline(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(out, "{}", self.0)
    }
}
pub fn raw(s: impl Into<Cow<'static, str>>) -> Raw {
    Raw(s.into())
}

#[macro_export]
macro_rules! cached_component {
    ($capacity:expr, $expr:expr) => {{
        static RENDERED: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let s = RENDERED.get_or_init(|| {
            let mut buf = String::with_capacity($capacity);
            ($expr).render_inline(&mut buf).unwrap();
            buf
        });
        raw(s.as_str())
    }};
    ($expr:expr) => {
        cached_component!(1024, $expr)
    };
}
