use crate::import_lib::import_lib;
use std::fs::read_to_string;

pub fn module(
    mut app: String,
    js: String
) -> (String, String) {

    let mut js_ = js;

    while app.contains("import mod:") {
        match app.find("import mod:") {
            None => {}
            Some(e) => {
                let mut ci = e + 9;

                while &app[ci..ci + 1] != "\n" {
                    ci += 1
                }

                let cloned = app.clone();

                let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();
                app.replace_range(e..ci + 1, "");

                for name in names {
                    let module = read_to_string(format!("./src/{name}"))
                        .unwrap_or_else(|_| panic!("Module {name} not found"));

                    js_ = import_lib(module.clone(), module.clone(), true).0;
                }

            }
        }
    }

    (app, js_)
}