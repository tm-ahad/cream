use crate::std_err::ErrType::CpuError;
use crate::std_err::StdErr;

pub fn cpu_error() {
    StdErr::exec(CpuError, "Cpu error at processing strings");
}
