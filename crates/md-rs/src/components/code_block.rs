use std::borrow::Cow;

use super::Component;

#[derive(Default)]
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
    pub fn content(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        self.content = content.into();
        self
    }
    pub fn language(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.lang = Some(lang.into());
        self
    }
}
impl Component for CodeBlock {
    fn render(&self, out: &mut String) {
        let fence = "```";
        let lang = self.lang.as_deref().unwrap_or("");
        out.push_str(&format!("{fence}{lang}\n{}\n{fence}\n\n", self.content));
    }
}

pub fn code_block() -> CodeBlock {
    CodeBlock::default()
}
