use crate::std_out::std_out;
use std::io;

pub fn std_input(p: &str, def: &str) -> String {
    std_out(p);

    let mut user_input = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.

    let _ = stdin.read_line(&mut user_input)
        .expect("Can read input from user");

    user_input = if user_input.trim().is_empty() {
        def.to_string()
    } else {user_input};

    user_input.trim().to_string()
}
