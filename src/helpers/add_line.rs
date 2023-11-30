pub fn add_line(refer: &mut String, line: &str) {
    refer.push_str(&format!("{line}\n"))
}
