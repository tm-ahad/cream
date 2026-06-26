use std::process::exit;
use serde_json::{Map, Value};
use crate::std_err::{ErrType, StdErr};

pub fn build_source(path: &str) -> String {
    let key = if path == "error" {path} else {&path.replace("/", "_")};
    format!("./build/{key}.js")
}

fn naive_inverse_search(map: &Map<String, Value>, val: &str) -> String {
    let pass_arr = match map.get("build") {
        Some(v) => v,
        None => {
            StdErr::exec(ErrType::NotFound, "build in routes.json");
            exit(1)
        }
    };

    let routes = match map.get("routes").unwrap() {
        Value::Object(obj) => obj,
        _ => todo!()
    };

    match pass_arr {
        Value::Array(s) => {
            if s.contains(&Value::String(val.to_string())) {
                return val.to_string();
            }
        },
        _ => {
            StdErr::exec(crate::std_err::ErrType::SyntaxError, "@pass in routes.json must be an array");
            exit(1);
        }
    }

    for (key, value) in routes {
        if value == &Value::String(val.to_string()) {
            return key.to_string();
        }
    }

    StdErr::exec(crate::std_err::ErrType::NotFound, &format!("file {}", val));
    exit(1)
}

pub fn build_import(path: &str, router_map: &Map<String, Value>) -> String {
    let path = naive_inverse_search(router_map, path);
    let key = if path == "error" {&path} else {&path.replace("/", "_")};
    format!("./{key}.js")
}

