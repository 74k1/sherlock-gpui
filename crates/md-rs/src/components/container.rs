use std::fmt::{Result, Write};

use md_rs_derive::{ComponentConstructor, ParentComponent};

use super::Component;

#[derive(Default, ComponentConstructor, ParentComponent)]
pub struct Container {
    children: Vec<Box<dyn Component>>,
}
impl Container {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            children: Vec::with_capacity(cap),
        }
    }

    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
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
