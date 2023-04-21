use crate::js_lib::libs;

pub fn import_lib(mut app: String, js: String, bind: bool) -> (String, String) {
    let mut js_ = js;

    while app.contains("import lib:") {
        match app.find("import lib:") {
            None => {}
            Some(e) => {
                let mut ci = e + 9;

                while &app[ci..ci + 1] != "\n" {
                    ci += 1
                }

                let name = &app.clone()[e + 11..ci];

                if !bind {
                    app.replace_range(e..ci + 1, "");
                } else {
                    js_.replace_range(e..ci + 1, "")
                }

                let resp = libs(name);
                js_ = format!("{resp}{js_}")
            }
        }
    }

    if !bind {(app, js_)}
    else {(js_, String::new())}

}