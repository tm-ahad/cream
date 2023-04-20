use crate::std_err::ErrType::CpuError;
use crate::std_err::StdErr;

pub fn cpu_error() {
    let error =
        StdErr::new(CpuError, "Cpu error at processing strings");

    error.exec();
}
