use super::Component;

#[derive(Default)]
pub struct ListItem {
    pub children: Vec<Box<dyn Component>>,
}
impl ListItem {
    pub fn child(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}
impl Component for ListItem {
    fn render(&self, out: &mut String) {
        out.push_str("- ");
        for child in &self.children {
            child.render(out);
        }
        out.push('\n');
    }
}
