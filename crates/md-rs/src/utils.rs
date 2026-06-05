use std::borrow::Cow;

use crate::components::{IntoComponent, span::Span, span_nodes::Paragraph};

impl IntoComponent for Cow<'static, str> {
    type Comp = Paragraph;
    fn into_component(self) -> Self::Comp {
        Paragraph {
            spans: vec![Span::Text(self)],
        }
    }
}

impl IntoComponent for String {
    type Comp = Paragraph;
    fn into_component(self) -> Self::Comp {
        Paragraph {
            spans: vec![Span::Text(self.into())],
        }
    }
}

impl IntoComponent for &'static str {
    type Comp = Paragraph;
    fn into_component(self) -> Self::Comp {
        Paragraph {
            spans: vec![Span::Text(self.into())],
        }
    }
}
