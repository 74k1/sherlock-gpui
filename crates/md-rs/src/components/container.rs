use std::fmt::{Result, Write};

use md_rs_derive::ComponentConstructor;

use crate::components::IntoComponent;

use super::Component;

#[derive(Default, ComponentConstructor)]
pub struct Container {
    children: Vec<Box<dyn Component>>,
}
impl Container {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            children: Vec::with_capacity(cap),
        }
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

impl Component for Container {
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        for child in &self.children {
            child.render(out)?;
        }

        Ok(())
    }
    fn render(&self, out: &mut dyn Write) -> Result {
        self.render_inline(out)
    }
}
