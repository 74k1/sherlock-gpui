pub mod code_block;
pub mod container;
pub mod heading;
pub mod hr;
pub mod list_item;
pub mod span;
pub mod span_nodes;
pub mod table;

// Traits
pub trait Component {
    fn render(&self, out: &mut String);
}
