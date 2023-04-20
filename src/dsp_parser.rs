use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn dsp_parser(path: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let s = match read_to_string(path) {
        Ok(a) => a,
        Err(e) => {
            StdErr::exec(OSError, &*e.to_string());
            todo!()
        }
    };

    for ln in s.clone().lines() {
        let pair = ln.split("$")
            .collect::<Vec<&str>>();

        map.insert(pair[0].to_string(),
                   pair[1].to_string());
    }

    map.clone()
}
