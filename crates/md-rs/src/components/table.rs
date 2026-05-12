use super::Component;
use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

#[derive(Default)]
pub struct Table {
    headers: Vec<Cow<'static, str>>,
    rows: Vec<Vec<Cow<'static, str>>>,
}

impl Table {
    pub fn headers(
        mut self,
        headers: impl IntoIterator<Item = impl Into<Cow<'static, str>>>,
    ) -> Self {
        self.headers = headers.into_iter().map(|h| h.into()).collect();
        self
    }

    pub fn row(mut self, row: impl IntoIterator<Item = impl Into<Cow<'static, str>>>) -> Self {
        self.rows.push(row.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn rows(
        mut self,
        rows: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<Cow<'static, str>>>>,
    ) -> Self {
        for row in rows {
            self = self.row(row);
        }
        self
    }
}

impl Component for Table {
    fn render(&self, out: &mut dyn Write) -> Result {
        if self.headers.is_empty() {
            return Ok(());
        }

        // header row
        write!(out, "|")?;
        for h in &self.headers {
            write!(out, " {} |", h)?;
        }
        writeln!(out)?;

        // separator
        write!(out, "|")?;
        for _ in &self.headers {
            write!(out, "---|")?;
        }
        writeln!(out)?;

        // rows
        for row in &self.rows {
            write!(out, "|")?;
            for cell in row {
                write!(out, " {} |", cell)?;
            }
            writeln!(out)?;
        }

        writeln!(out)
    }
}

pub fn table() -> Table {
    Table::default()
}
