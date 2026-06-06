use std::fmt::{Result, Write};

use md_rs_derive::{ComponentBuilder, ComponentConstructor, ParentComponent};

use crate::components::{Component, IntoComponent, span_nodes::Paragraph};

#[derive(Default, ComponentConstructor, ParentComponent)]
pub struct ListItem {
    children: Vec<Box<dyn Component>>,
}
impl ListItem {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            children: Vec::with_capacity(cap),
        }
    }
}

impl<C: IntoComponent + 'static> From<C> for ListItem {
    fn from(value: C) -> Self {
        Self {
            children: vec![Box::new(value.into_component())],
        }
    }
}

#[derive(Default, ComponentConstructor, ComponentBuilder)]
#[md_rs(rename = "md_list")]
pub struct List {
    title: Option<Paragraph>,
    style: ListStyle,
    #[md_rs(skip_builder)]
    items: Vec<ListItem>,
}
impl List {
    pub fn item(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
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

#[macro_export]
macro_rules! item {
    ($($child:expr),* $(,)?) => {
        {
            const COUNT: usize = [$( { _ = stringify!($child); 1 } ),*].len();
            let mut list_item = ::md_rs::components::list::ListItem::with_capacity(COUNT);
            $(
                list_item = list_item.child($child);
            )*
            list_item
        }
    }
}

#[macro_export]
macro_rules! list {
    ($style:ident, $($item:expr),+ $(,)?) => {
        {
            let mut l = ::md_rs::components::list::List::default()
                .style(::md_rs::components::list::ListStyle::$style);
            $(
                l = l.item($item);
            )*
            l
        }
    };
}

#[macro_export]
macro_rules! list_iter {
    ($style:ident, $iter:expr $(,)?) => {
        ::md_rs::components::list::List::default()
            .style(::md_rs::components::list::ListStyle::$style)
            .items($iter)
    };
}
