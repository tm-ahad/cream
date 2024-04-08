use std::fmt::Display;
use colored::Colorize;
use std::process::exit;

pub struct StdErr;

pub enum ErrType {
    PackageError,
    LibraryError,
    SyntaxError,
    NetError,
    OSError
}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = String::from(match self {
            ErrType::PackageError => "PackageError",
            ErrType::LibraryError => "LibraryError",
            ErrType::SyntaxError => "SyntaxError",
            ErrType::NetError => "NetError",
            ErrType::OSError => "OSError"
        });

        write!(f, "{}", str)
    }
}

impl StdErr {
    pub fn exec(type_: ErrType, err: &str) {
        let (r, g, b) = (242, 53, 19);
        let error = err.truecolor(r, g, b);

        eprintln!("{}: {}", type_.to_string(), error);
        exit(1);
    }
}
