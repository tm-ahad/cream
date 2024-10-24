use crate::consts::{NEW_LINE_CHAR, NIL};
use crate::helpers::javascript_function_call::javascript_function_call;
use crate::component::component;
use crate::javascript_lib::libs;
use std::fs::read_to_string;
use serde_json::{Map, Value};
use crate::component_map::ComponentMap;
use crate::out::out;

pub fn router(component_map: &mut ComponentMap) -> String {
    let path = "src/routes.json";

    let raw = match read_to_string(path) {
        Ok(content) => content,
        Err(_) => return String::new(),
    };

    let val = match serde_json::from_str::<Value>(&raw) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };

    if let Value::Object(map) = val {
        let mut cmap = Map::new();

        for (key, f_name) in map {
            if let Value::String(f_name) = f_name {
                let comp = component(
                    f_name.clone(),
                    String::from("Page"),
                    component_map,
                );

                let script = comp.script.replace("\n\n", NIL);
                let html = comp.html.stat.replace("\n\n", NIL);

                cmap.insert(key, Value::Array(vec![Value::String(html.clone()), Value::String(script.clone())]));

                let config = component_map.config();
                out(&format!("./build/routes/{f_name}"), html, script, config)
            }
        }

        let enc = serde_json::to_string(&cmap).unwrap();
        let mut _router = libs("router.js", true);
        _router.push(NEW_LINE_CHAR);

        return format!(
            "{}\n{}\n",
            _router,
            javascript_function_call("router", vec![enc])
        )
    }

    String::new()
}
