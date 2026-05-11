use super::Component;
use std::borrow::Cow;

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
    fn render(&self, out: &mut String) {
        if self.headers.is_empty() {
            return;
        }

        // header row
        out.push('|');
        for h in &self.headers {
            out.push_str(&format!(" {} |", h));
        }
        out.push('\n');

        // separator
        out.push('|');
        for _ in &self.headers {
            out.push_str("---|");
        }
        out.push('\n');

        // rows
        for row in &self.rows {
            out.push('|');
            for cell in row {
                out.push_str(&format!(" {} |", cell));
            }
            out.push('\n');
        }

        out.push('\n');
    }
}

pub fn table() -> Table {
    Table::default()
}
