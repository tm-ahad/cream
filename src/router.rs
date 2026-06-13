use crate::transpiler::transpile_component_;
use crate::component_map::ComponentMap;
use std::{fs::{self, read_to_string}, io::Error, path::Path};
use serde_json::{Map, Value};
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

pub fn router(component_map: &mut ComponentMap) {
    let bind = component_map.config();
    let path = bind.expect("routes");

    let _ = clear_dir(Path::new("./build"));
    let raw = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => panic!("cannot open router"),
    };

    let val = match serde_json::from_str::<Value>(&raw) {
        Ok(v) => v,
        Err(_) => panic!("invalid router json"),
    };

    if let Value::Object(map) = val {
        let mut cmap = Map::new();

        for (key, f_name) in map {
            if let Value::String(f_name) = f_name {
                let comp = transpile_component_(
                    &mut component_map.component_args.import_base,
                    &component_map.component_args.config,
                    f_name.clone(),
                    String::from("Page")
                );

                let script = comp.script;
                let html = comp.html;

                cmap.insert(key.clone(), Value::Array(vec![Value::String(html.clone()), Value::String(script.clone())]));

                let config = component_map.config();
                let key = if key == "error" {key} else {key.replace("/", ":")};
                out(&format!("./build/{key}", ), &f_name, html, script, &config)
            }
        }
    }

}
