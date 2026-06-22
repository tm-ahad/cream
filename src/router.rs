use crate::consts::BUILD_PATH;
use crate::helpers::dependancy_graph::DependancyGraph;
use crate::std_err::{ErrType, StdErr};
use crate::{component::Component, dsp_map::DspMap};
use crate::helpers::build_source::build_source;
use std::process::exit;
use std::{fs::{self, read_to_string}, io::Error, path::Path};
use serde_json::Value;
use crate::out::out;

fn clear_dir(path: &Path) -> Result<(), Error> {
    if !path.is_dir() {
        return Ok(())
    }

    for child in fs::read_dir(path)? {
        let path = child?.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

pub fn router(conf: &mut DspMap) {
    let path = conf.expect("project");
    let _ = clear_dir(Path::new("./build"));
    
    let raw = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => panic!("cannot open router"),
    };

    let val = match serde_json::from_str::<Value>(&raw) {
        Ok(Value::Object(obj)) => obj,
        _ => {
            StdErr::exec(ErrType::SyntaxError, "router.json must contain an object");
            exit(1)
        }
    };

    if let Value::Array(v) = val.get("build").unwrap_or_else(|| {
        StdErr::exec(ErrType::NotFound, "property build in routes.json");
        exit(1)
    }) {
        for prop_val in v {
            let str_val = match prop_val {
                Value::String(val_) => val_,
                _ => {
                    StdErr::exec(ErrType::SyntaxError, "build must only contain strings");
                    exit(1);
                }
            };

            let mut dep_graph = DependancyGraph::new();
            let mut comp = Component::new(String::new(), str_val.to_string(), &val, &mut dep_graph);
            comp.transpile();
            out(&build_source(str_val), comp.out, conf);
            dep_graph.install(BUILD_PATH);
        }
    }

    if let Value::Object(map) = val.get("routes").unwrap_or_else(|| {
        StdErr::exec(ErrType::NotFound, "routes in routes.json");
        exit(1)
    }) {
        for (key, f_name) in map.iter() {
            if let Value::String(f_name) = f_name {
                let mut dep_graph = DependancyGraph::new();
                let mut comp = Component::new(String::new(), f_name.to_string(), &val, &mut dep_graph);
                comp.transpile();
                out(&build_source(key), comp.out, conf);
                dep_graph.install(BUILD_PATH);
            }
        }
    }

}
