use crate::helpers::is_byte_in_str::is_byte_in_str;
pub fn escape_string(s: &mut String) {
    let mut escaped = String::new();
    let chars = s.chars();

    for (i, c) in chars.enumerate() {
        if is_byte_in_str(i, s) {
            match c {
                '"' => escaped.push_str("\\\""),
                '\'' => escaped.push_str("\\'"),
                '`' => escaped.push_str("\\`"),
                _ => escaped.push(c),
            }
        } else {
            escaped.push(c);
        }
    }

    *s = escaped;
}
