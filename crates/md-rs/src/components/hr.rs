use std::fmt::{Result, Write};

use md_rs_derive::ComponentConstructor;

use crate::components::Component;

#[derive(Default, ComponentConstructor)]
pub struct Hr;

impl Component for Hr {
    fn is_block(&self) -> bool {
        true
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        write!(out, "---")
    }
}
