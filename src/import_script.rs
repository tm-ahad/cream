use std::fs::read_to_string;

pub fn import_script(mut app: String, js: String) -> (String, String) {
    let mut js_ = js;

    while app.contains("import script:") {
        match app.find("import script:") {
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
                    let resp = read_to_string(name)
                        .unwrap_or_else(|_| panic!("Script {name} not found"));
                    js_ = format!("{resp}{js_}")
                }


            }
        }
    }

    (app, js_)
}