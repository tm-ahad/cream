use colored::Colorize;
use std::process::exit;

pub struct StdErr;

pub enum ErrType {
    PackageError,
    LibraryError,
    SyntaxError,
    OSError,
}

impl ToString for ErrType {
    fn to_string(&self) -> String {
        String::from(match self {
            ErrType::PackageError => "PackageError",
            ErrType::LibraryError => "LibraryError",
            ErrType::SyntaxError => "SyntaxError",
            ErrType::OSError => "OSError",
        })
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
