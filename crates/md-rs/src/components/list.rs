use std::fmt::{Result, Write};

use md_rs_derive::ComponentConstructor;

use crate::components::{Component, IntoComponent, span_nodes::Paragraph};

#[derive(Default, ComponentConstructor)]
pub struct ListItem {
    children: Vec<Box<dyn Component>>,
}
impl ListItem {
    pub fn child(mut self, child: impl IntoComponent + 'static) -> Self {
        self.children.push(Box::new(child.into_component()));
        self
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

impl<C: IntoComponent + 'static> From<C> for ListItem {
    fn from(value: C) -> Self {
        Self {
            children: vec![Box::new(value.into_component())],
        }
    }
}

#[derive(Default, ComponentConstructor)]
pub struct List {
    title: Option<Paragraph>,
    items: Vec<ListItem>,
    style: ListStyle,
}
impl List {
    pub fn style(mut self, style: impl Into<ListStyle>) -> Self {
        self.style = style.into();
        self
    }

    pub fn title(mut self, title: impl Into<Paragraph>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn item(mut self, item: ListItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = impl Into<ListItem> + 'static>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }
}

impl Component for List {
    fn render(&self, out: &mut dyn Write) -> Result {
        self.render_inline(out)?;
        writeln!(out)
    }
    fn render_inline(&self, out: &mut dyn Write) -> Result {
        if let Some(title) = &self.title {
            title.render(out)?;
        }
        for (i, item) in self.items.iter().enumerate() {
            self.style.write_prefix(out, i)?;
            for (j, child) in item.children.iter().enumerate() {
                if j > 0 {
                    if child.is_block() {
                        write!(out, "\n\n{}", self.style.indent(i))?;
                    } else {
                        write!(out, "\n{}", self.style.indent(i))?;
                    }
                }
                let mut writer = IndentWriter {
                    inner: out,
                    indent: self.style.indent(i),
                    at_line_start: false,
                };
                child.render_inline(&mut writer)?;
                if child.is_block() && i != self.items.len() - 1 {
                    writeln!(out)?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum ListStyle {
    #[default]
    Ordered,
    Dash,
    Asterisk,
    Plus,
}

impl ListStyle {
    fn write_prefix(&self, out: &mut dyn Write, i: usize) -> Result {
        match self {
            ListStyle::Ordered => write!(out, "{}. ", i + 1),
            ListStyle::Dash => write!(out, "- "),
            ListStyle::Asterisk => write!(out, "* "),
            ListStyle::Plus => write!(out, "+ "),
        }
    }
    fn indent(&self, i: usize) -> &'static str {
        match self {
            ListStyle::Ordered => match i {
                0..=8 => "   ",
                9..=98 => "    ",
                _ => "     ",
            },
            ListStyle::Dash | ListStyle::Asterisk | ListStyle::Plus => "  ",
        }
    }
}

impl<C: AsRef<str>> From<C> for ListStyle {
    fn from(value: C) -> Self {
        match value.as_ref() {
            "-" => Self::Dash,
            "*" => Self::Asterisk,
            "+" => Self::Plus,
            _ => Self::Ordered,
        }
    }
}

struct IndentWriter<'a> {
    inner: &'a mut dyn Write,
    indent: &'static str,
    at_line_start: bool,
}

impl Write for IndentWriter<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            if self.at_line_start && c != '\n' {
                self.inner.write_str(self.indent)?;
                self.at_line_start = false;
            }
            self.inner.write_char(c)?;
            if c == '\n' {
                self.at_line_start = true;
            }
        }
        Ok(())
    }
}
