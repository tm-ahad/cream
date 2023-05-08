use std::io::{stdout, Write};

pub fn std_out(input: &str) {
    let bytes = input.as_bytes();

    let _ = stdout().write_all(bytes);
    stdout().flush().unwrap();
}
