use std::fmt::{Result, Write};

use crate::components::span_nodes::Paragraph;

use super::Component;
use md_rs_derive::ComponentConstructor;

#[derive(Default, ComponentConstructor)]
pub struct Details {
    summary: Paragraph,
    children: Vec<Box<dyn Component>>,
}

impl Details {
    pub fn summary(mut self, summary: Paragraph) -> Self {
        self.summary = summary;
        self
    }

    pub fn child(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Component + 'static>,
    ) -> Self {
        self.children.extend(
            children
                .into_iter()
                .map(|c| Box::new(c) as Box<dyn Component>),
        );
        self
    }
}

impl Component for Details {
    fn render(&self, out: &mut dyn Write) -> Result {
        writeln!(out, "<details>")?;

        writeln!(out, "<summary>")?;
        self.summary.render(out)?;
        writeln!(out, "</summary>")?;
        writeln!(out)?;

        for child in &self.children {
            child.render(out)?;
        }

        writeln!(out, "</details>")?;
        writeln!(out)
    }
}
