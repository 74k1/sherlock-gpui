use std::fmt::{Result, Write};

pub mod code_block;
pub mod container;
pub mod heading;
pub mod hr;
pub mod list;
pub mod raw;
pub(crate) mod span;
pub mod span_nodes;
pub mod table;

#[cfg(feature = "github")]
pub mod details;

// Traits
pub trait Component {
    fn render_inline(&self, out: &mut dyn Write) -> Result;

    fn render(&self, out: &mut dyn Write) -> Result {
        self.render_inline(out)?;
        writeln!(out, "\n")
    }

    fn is_block(&self) -> bool {
        false
    }
}

impl<C: Component> Component for &C {
    fn render(&self, out: &mut dyn Write) -> Result {
        (*self).render(out)
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        (*self).render_inline(out)
    }
}

pub trait IntoComponent {
    type Comp: Component;
    fn into_component(self) -> Self::Comp;
}

impl<C: Component> IntoComponent for C {
    type Comp = C;
    fn into_component(self) -> Self::Comp {
        self
    }
}
