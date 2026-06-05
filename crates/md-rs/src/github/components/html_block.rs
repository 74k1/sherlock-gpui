use std::borrow::Cow;

use md_rs_derive::{ComponentBuilder, ComponentConstructor, ParentComponent};

use crate::components::Component;

#[derive(Default)]
pub enum Orientation {
    #[default]
    Left,
    Center,
    Right,
}

impl<S: AsRef<str>> From<S> for Orientation {
    fn from(value: S) -> Self {
        match value.as_ref() {
            "left" => Orientation::Left,
            "center" => Orientation::Center,
            "right" => Orientation::Right,
            _ => Self::default(),
        }
    }
}

#[derive(Default, ComponentConstructor, ParentComponent, ComponentBuilder)]
pub struct HtmlBlock {
    orientation: Orientation,
    style: Cow<'static, str>,
    #[md_rs(skip_builder)]
    children: Vec<Box<dyn Component>>,
}

impl Component for HtmlBlock {
    fn is_block(&self) -> bool {
        true
    }

    fn render_inline(&self, out: &mut dyn std::fmt::Write) -> std::fmt::Result {
        let align = match self.orientation {
            Orientation::Left => "left",
            Orientation::Center => "center",
            Orientation::Right => "right",
        };
        writeln!(out, r#"<div align="{align}" style="{}">"#, self.style)?;

        for child in &self.children {
            child.render(out)?;
        }
        write!(out, "</div>")
    }
}
