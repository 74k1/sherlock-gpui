use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use md_rs_derive::{ComponentBuilder, ComponentConstructor};

use super::Component;

#[derive(Default, ComponentConstructor, ComponentBuilder)]
pub struct CodeBlock {
    pub lang: Option<Cow<'static, str>>,
    pub content: Cow<'static, str>,
}
impl CodeBlock {
    pub fn line(mut self, line: impl AsRef<str>) -> Self {
        let content = self.content.to_mut();
        if !content.is_empty() {
            content.push('\n');
        }
        content.push_str(line.as_ref());
        self
    }
}
impl Component for CodeBlock {
    fn is_block(&self) -> bool {
        true
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        let fence = "```";
        let lang = self.lang.as_deref().unwrap_or("");
        writeln!(out, "{fence}{lang}")?;
        writeln!(out, "{}", self.content.trim_end())?;
        write!(out, "{fence}")
    }
}
