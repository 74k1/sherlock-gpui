use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use crate::components::span::{LinkData, Span};

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

pub trait ParentComponentExt: Sized {
    fn child(self, child: impl IntoComponent + 'static) -> Self;
    fn children(self, children: impl IntoIterator<Item = impl IntoComponent + 'static>) -> Self;
}

pub trait TextComponentExt {
    fn spans_mut(&mut self) -> &mut Vec<Span>;

    fn br(mut self) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::LineBreak);
        self
    }
    fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Text(text.into()));
        self
    }
    fn italic(mut self, text: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Italic(text.into()));
        self
    }
    fn bold(mut self, text: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Bold(text.into()));
        self
    }
    fn code(mut self, text: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Code(text.into()));
        self
    }
    fn link(
        mut self,
        title: impl Into<Cow<'static, str>>,
        target: impl Into<Cow<'static, str>>,
    ) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Link(Box::new(LinkData {
            title: vec![Span::Text(title.into())],
            target: target.into(),
        })));
        self
    }
    fn link_bold(
        mut self,
        title: impl Into<Cow<'static, str>>,
        target: impl Into<Cow<'static, str>>,
    ) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::Link(Box::new(LinkData {
            title: vec![Span::Bold(title.into())],
            target: target.into(),
        })));
        self
    }

    #[cfg(feature = "github")]
    fn html_strong(mut self, text: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.spans_mut().push(Span::HtmlStrong(text.into()));
        self
    }
}
