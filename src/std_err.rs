use std::process::exit;
use colored::Colorize;

pub struct StdErr;

#[allow(dead_code)]
pub enum ErrType {
    SyntaxError,
    CpuError,
    OSError,
    ConfigError,
    NotFound
}

impl ErrType {
    pub fn _to_string(self) -> String {
        match self {
            ErrType::SyntaxError => "SyntaxError",
            ErrType::CpuError => "CpuError",
            ErrType::OSError => "OSError",
            ErrType::NotFound => "NotFound",
            _ => "ConfigError",
        }.to_string()
    }
}

impl StdErr {

    pub fn exec(type_: ErrType, err: &str) {
        println!(
            "{}: {}",
            type_._to_string(),
            err.truecolor(242, 53, 19)
        );

        exit(400)
    }
}
