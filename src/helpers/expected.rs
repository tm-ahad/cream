use crate::std_err::{self, StdErr};

pub fn expect_some<T>(val: Option<T>, expectation: &str) -> T {
    match val {
        Some(val) => val,
        None => {
            StdErr::exec(std_err::ErrType::NotFound, expectation);
            todo!()
        },
    }
}
