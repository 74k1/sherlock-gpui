pub mod binds;
pub mod exec_mode;

pub fn to_title_case(input_str: &str) -> String {
    let mut result = String::with_capacity(input_str.len());
    let mut cap_next = true;
    for c in input_str.chars() {
        if c.is_whitespace() {
            cap_next = true;
            result.push(c);
        } else if cap_next {
            for up in c.to_uppercase() {
                result.push(up)
            }
            cap_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
