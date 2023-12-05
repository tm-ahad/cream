use crate::dsp_map::DspMap;
use crate::consts::NEW_LINE_CHAR;
use crate::helpers::javascript_function_call::javascript_function_call;
use crate::std_err::ErrType::PackageError;
use crate::std_err::StdErr;
use std::fs::read_to_string;
use ureq::{get, Response};

pub fn router(config: &DspMap) -> String{
    let path = format!("./{}", config.expect("routes"));
    let mut raw = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => return String::new(),
    };

    if raw.is_empty() {
        raw = String::from("{}");
    }

    let url = "https://raw.githubusercontent.com/tm-ahad/cream/master/scripts/router.js";

    let resp = get(&url).call().unwrap_or_else(|e| {
        StdErr::exec(PackageError, &e.to_string());
        Response::new(404, "PackageError", "").unwrap()
    });

    return if resp.status() == 200 {
        let mut scr = resp.into_string().unwrap_or_else(|e| panic!("{e}"));
        scr.push(NEW_LINE_CHAR);

        format!("{}{}",
            scr,
            javascript_function_call("router", vec![raw])
        )
    } else {
        StdErr::exec(PackageError, &format!("{url} not found"));
        todo!()
    }
}
