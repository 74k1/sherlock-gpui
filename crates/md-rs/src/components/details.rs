use std::fmt::{Result, Write};

use crate::components::{IntoComponent, span_nodes::Paragraph};

use super::Component;
use md_rs_derive::ComponentConstructor;

#[derive(Default, ComponentConstructor)]
pub struct Details {
    summary: Paragraph,
    children: Vec<Box<dyn Component>>,
}

impl Details {
    pub fn summary(mut self, summary: impl Into<Paragraph>) -> Self {
        self.summary = summary.into();
        self
    }

    pub fn child(mut self, child: impl IntoComponent + 'static) -> Self {
        self.children.push(Box::new(child.into_component()));
        self
    }

    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl IntoComponent + 'static>,
    ) -> Self {
        self.children.extend(
            children
                .into_iter()
                .map(|c| Box::new(c.into_component()) as Box<dyn Component>),
        );
        self
    }
}

impl Component for Details {
    fn is_block(&self) -> bool {
        true
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        writeln!(out, "<details>")?;
        write!(out, "<summary>")?;
        self.summary.render_inline(out)?;
        writeln!(out, "</summary>")?;
        writeln!(out)?; // blank line required by GFM after </summary>
        for child in &self.children {
            child.render(out)?; // each child self-terminates with \n
        }
        write!(out, "</details>") // no trailing \n
    }
}
