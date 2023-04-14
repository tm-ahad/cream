use std::process::exit;
use colored::Colorize;

#[allow(dead_code)]
pub enum ErrType {
    SyntaxError,
    CpuError,
    OSError,
    ConfigError
}

impl ErrType {
    pub fn _to_string(self) -> String {
        match self {
            ErrType::SyntaxError => "SyntaxError".to_string(),
            ErrType::CpuError => "CpuError".to_string(),
            ErrType::OSError => "OSError".to_string(),
            _ => "ConfigError".to_string()
        }
    }
}

pub struct StdErr {
    err: String,
    type_: ErrType,
}

impl StdErr {
    pub fn new(type_: ErrType, e: &str) -> StdErr {
        StdErr {
            err: e.to_string(),
            type_,
        }
    }

    pub fn exec(self) {
        println!(
            "{}: {}",
            self.type_._to_string(),
            self.err.truecolor(242, 53, 19)
        );

        exit(400)
    }
}
