use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

use md_rs_derive::{ComponentBuilder, ComponentConstructor};

use crate::components::Component;

#[derive(Default, ComponentConstructor, ComponentBuilder)]
pub struct Badge {
    alt: Cow<'static, str>,
    image_url: Cow<'static, str>,
    link: Option<Cow<'static, str>>,
}

impl Component for Badge {
    fn is_block(&self) -> bool {
        false
    }

    fn render(&self, out: &mut dyn Write) -> Result {
        self.render_inline(out)?;
        writeln!(out)
    }

    fn render_inline(&self, out: &mut dyn Write) -> Result {
        if let Some(link) = &self.link {
            write!(out, "[![{}]({})]({})", self.alt, self.image_url, link)
        } else {
            write!(out, "![{}]({})", self.alt, self.image_url)
        }
    }
}
