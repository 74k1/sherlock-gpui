use std::fmt::{Result, Write};

use md_rs_derive::ComponentConstructor;

use super::Component;

#[derive(Default, ComponentConstructor)]
pub struct ListItem {
    pub children: Vec<Box<dyn Component>>,
}
impl ListItem {
    pub fn child(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}
impl Component for ListItem {
    fn render(&self, out: &mut dyn Write) -> Result {
        write!(out, "- ")?;
        for child in &self.children {
            child.render(out)?;
        }
        writeln!(out)
    }
}
