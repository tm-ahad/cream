use std::io::{BufRead, self, Write};


pub fn std_input(p: &str, def: &str) -> String {
    print!("{}", p);
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    let stdin = io::stdin();

    let _ = stdin.lock().read_line(&mut user_input);

    user_input = if user_input.trim().is_empty() {
        def.to_string()
    } else {
        user_input
    };

    user_input.trim().to_string()
}
