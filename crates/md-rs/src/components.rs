use std::fmt::{Result, Write};

pub mod code_block;
pub mod container;
pub mod heading;
pub mod hr;
pub mod list_item;
pub mod span;
pub mod span_nodes;
pub mod table;

#[cfg(feature = "github")]
pub mod details;

// Traits
pub trait Component {
    fn render(&self, out: &mut dyn Write) -> Result;
}
