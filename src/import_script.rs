use crate::import_base::ImportBase;
use std::fs::read_to_string;
use crate::import_base::ImportType::Scripts;

pub fn import_script(mut app: String, import_base: &mut ImportBase, js: String) -> (String, String) {
    let mut js_ = js;

    while let Some(e) = app.find("import script:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();

        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();
        app.replace_range(e..ci + 1, "");

        for name in names {

            if import_base.validate(Scripts, name.to_string()) {
                let fmt = format!("./src/{name}");
                let resp = read_to_string(fmt.to_string())
                    .unwrap_or_else(|_| panic!("Script {name} not found"));

                import_base.push(Scripts, fmt.to_string());

                js_ = format!("{resp}{js_}")
            }
        }
    }

    (app, js_)
}