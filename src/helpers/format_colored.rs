pub fn format_colored(s: &str, r: u8, g: u8, b: u8) -> String {

    return format!("\x1b[38;2;{};{};{}m{s}\x1b[0m", r, g, b);
}
