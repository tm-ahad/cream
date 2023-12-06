use std::collections::HashMap;
use crate::dsp_map::DspMap;
use crate::consts::{NEW_LINE, NEW_LINE_CHAR, NIL};
use crate::helpers::javascript_function_call::javascript_function_call;
use crate::std_err::ErrType::PackageError;
use crate::component::component;
use crate::std_err::StdErr;
use std::fs::read_to_string;
use serde_json::Value;
use ureq::{get, Response};

pub fn router(config: &DspMap) -> String{
    let path = format!("./{}", config.expect("routes"));
    let raw = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => return String::new(),
    };

    let val = match serde_json::from_str::<Value>(&raw) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };

    if let Value::Object(map) = val {
        let mut cmap = HashMap::new();

        for (key, value) in map {
            if let Value::String(value) = value {
                cmap.insert(key, component(
                    &value,
                    "Page",
                    config.expect("build"),
                    config
                )
                    .static_transpiled()
                    .replace(NEW_LINE, NIL)
                );
            }
        }

        let enc = format!("{:?}", cmap);
        let url = "https://raw.githubusercontent.com/tm-ahad/cream/master/scripts/router.js";

        let resp = get(&url).call().unwrap_or_else(|e| {
            StdErr::exec(PackageError, &e.to_string());
            Response::new(404, "PackageError", "").unwrap()
        });

        return if resp.status() == 200 {
            let mut scr = resp.into_string().unwrap_or_else(|e| panic!("{e}"));
            scr.push(NEW_LINE_CHAR);

            format!("{}\n{}\n",
                    scr,
                    javascript_function_call("router", vec![enc])
            )
        } else {
            StdErr::exec(PackageError, &format!("{url} not found"));
            todo!()
        }
    }

    return String::new();
}
