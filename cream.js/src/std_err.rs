use std::process::exit;
use colored::Colorize;

pub struct StdErr;

pub enum ErrType {
    SyntaxError,
    OSError,
    ConfigError,
    NotFound,
    PackageError,
}

impl ErrType {
    pub fn _to_string(self) -> String {
        String::from(match self {
            ErrType::SyntaxError => "SyntaxError",
            ErrType::OSError => "OSError",
            ErrType::NotFound => "NotFound",
            ErrType::PackageError => "PackageError",
            _ => "ConfigError",
        })
    }
}

impl StdErr {

    pub fn exec(type_: ErrType, err: &str) {
        eprintln!(
            "{}: {}",
            type_._to_string(),
            err.truecolor(242, 53, 19)
        );

        exit(400)
    }
}
