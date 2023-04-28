use crate::import_base::ImportBase;
use crate::import_base::ImportType::Libs;
use crate::js_lib::libs;

pub fn import_lib(app: String, import_base: &mut ImportBase, js: String, bind: bool) -> (String, String) {
    let js_ = js;
    let mut find = if bind {
        js_.clone()
    } else {app.clone()};

    while let Some(e) = find.find("import lib:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        find.replace_range(e..ci + 1, "");

        for name in names {

            if import_base.validate(Libs, name.to_string()) {
                let resp = libs(name);
                import_base.push(Libs, name.to_string());

                find.insert_str(0, &resp)
            }
        }
    }

    if !bind {(find, js_)}
    else {(find, String::new())}

}