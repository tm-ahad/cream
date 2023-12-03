use crate::consts::{DEFAULT_ERROR_RESPONSE, ERROR_COMPONENT, RENDER_COMPONENT, ROUTER_TOKEN, ROUTER_TOKEN_LEN, SBF_AS_STR};
use crate::helpers::router_script::router_script;
use crate::helpers::component_part::ComponentPart;
use crate::component_markup::ComponentMarkUp;
use crate::component_args::ComponentArgs;
use crate::component::component;
use crate::out::out_to_error;
use crate::pass::pass;
use serde_json::{Map, Value};
use crate::helpers::read_until::read_until;

pub fn router(
    html: &mut ComponentMarkUp,
    script: &mut String,
    component_args: &ComponentArgs,
    f_name: &str
) {
    match html.stat.find(ROUTER_TOKEN) {
        None => pass(),
        Some(a) => {
            if &html.stat[a + ROUTER_TOKEN_LEN..a + ROUTER_TOKEN_LEN + 1] == SBF_AS_STR {
                let idx = read_until(&html.stat, a+ROUTER_TOKEN_LEN, SBF_AS_STR, f_name, ComponentPart::Unknown);
                let json = &html.stat[a + ROUTER_TOKEN_LEN..idx + 1];

                match serde_json::from_str::<Value>(json) {
                    Ok(a) => {
                        let obj = a.as_object().unwrap();

                        let mut map = Map::new();

                        for (key, val) in obj {
                            let s = val.as_str().unwrap();

                            let comp = component(
                                s,
                                RENDER_COMPONENT,
                                component_args.transpile_command,
                                component_args.config,
                            );

                            let fin = format!("{}\n{}", router_script(), comp.static_transpiled());

                            map.insert(key.clone(), Value::String(fin));
                        }

                        script.push_str(&format!(
                            "\nvar Route = {}",
                            serde_json::to_string::<Value>(&Value::Object(map)).unwrap()
                        ));
                    }
                    Err(_) => panic!("Can't even parse json in ohio"),
                }

                html.stat.replace_range(a..idx + 2, "")
            } else {
                let not_found = match component_args.config.get("404") {
                    Some(e) => {
                        component(
                            e,
                            ERROR_COMPONENT,
                            component_args.transpile_command,
                            component_args.config
                        ).static_transpiled()
                    }
                    None => String::from(DEFAULT_ERROR_RESPONSE),
                };

                out_to_error(&not_found);
            }
        }
    }
}
