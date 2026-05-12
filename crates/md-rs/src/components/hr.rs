use std::fmt::{Result, Write};

use md_rs_derive::ComponentConstructor;

use crate::components::Component;

#[derive(Default, ComponentConstructor)]
pub struct Hr;
impl Component for Hr {
    fn render(&self, out: &mut dyn Write) -> Result {
        writeln!(out, "---")?;
        writeln!(out)
    }
}
