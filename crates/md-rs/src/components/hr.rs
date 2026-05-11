use crate::components::Component;

pub struct Hr;
impl Component for Hr {
    fn render(&self, out: &mut String) {
        out.push_str("---\n\n");
    }
}

pub fn hr() -> Hr {
    Hr {}
}
