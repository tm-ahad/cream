use crate::helpers::format_colored::format_colored;
use std::fmt::Display;

pub struct StdErr;

pub enum ErrType {
    LibraryError,
    SyntaxError,
    NotFound,
    OSError
}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from(match self {
            ErrType::LibraryError => "Library error",
            ErrType::SyntaxError => "Syntax error",
            ErrType::OSError => "OS error",
            ErrType::NotFound => "Can't find",
        });

        write!(f, "{}", str)
    }
}

impl StdErr {
    pub fn exec(type_: ErrType, err: &str) {
        let (r, g, b) = (242, 53, 19);
        let error = format_colored(err, r, g, b);
        eprintln!("{}: {}", type_, error);
    }
}
