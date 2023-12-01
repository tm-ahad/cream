use colored::Colorize;
use std::process::exit;

pub struct StdErr;

pub enum ErrType {
    OSError,
    PackageError,
    LibraryError,
}

impl ToString for ErrType {
    fn to_string(&self) -> String {
        String::from(match self {
            ErrType::OSError => "OSError",
            ErrType::PackageError => "PackageError",
            ErrType::LibraryError => "LibraryError",
        })
    }
}

impl StdErr {
    pub fn exec(type_: ErrType, err: &str) {
        eprintln!("{}: {}", type_.to_string(), err.truecolor(242, 53, 19));

        exit(400);
    }
}
