pub const NULL: &str = "null";

pub fn javascript_init_var(name: &str, value: &str) -> String {
    format!(";var {name}={value};")
}
