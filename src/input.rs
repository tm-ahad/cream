use crate::std_out::std_out;
use std::io::stdin;

pub fn std_input(prompt: &str, def: &str) -> String {
    std_out(&*format!("{prompt} ({def}): "));

    let mut user_input = String::new();

    stdin().read_line(&mut user_input)
        .expect("TODO: input cannot be read");

    if user_input.replace(" ", "") == "" {
        user_input = def.to_string()
    }

    user_input
}