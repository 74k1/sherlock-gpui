use super::Component;

#[derive(Default)]
pub struct Container {
    children: Vec<Box<dyn Component>>,
}
impl Container {
    pub fn child(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Component + 'static>,
    ) -> Self {
        self.children.extend(
            children
                .into_iter()
                .map(|c| Box::new(c) as Box<dyn Component>),
        );
        self
    }
}

impl Component for Container {
    fn render(&self, out: &mut String) {
        for child in &self.children {
            child.render(out);
        }
    }
}
