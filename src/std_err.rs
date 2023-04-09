use colored::Colorize;

#[allow(dead_code)]
pub enum ErrType {
    SyntaxError,
    NotFound,
    CpuError
}

impl ErrType {
    pub fn _to_string(self) -> String {
        match self {
            ErrType::SyntaxError => "Syntax Error".to_string(),
            ErrType::NotFound => "Not Found".to_string(),
            ErrType::RamError => "CpuError".to_string()
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
        )
    }
}
