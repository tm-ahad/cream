use crate::import_base::ImportBase;
use crate::import_base::ImportType::Libs;
use crate::js_lib::libs;

pub fn import_lib(app: &mut String, import_base: &mut ImportBase, js: &mut String) {

    while let Some(e) = app.find("import lib:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {

            if import_base.validate(Libs, name.to_string()) {
                let resp = libs(name);
                import_base.push(Libs, name.to_string());

                js.insert_str(0, &resp)
            }
        }
    }
}

pub fn import_lib_bind(app: &mut String, import_base: &mut ImportBase) {

    while let Some(e) = app.find("import lib:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {

            if import_base.validate(Libs, name.to_string()) {
                let resp = libs(name);
                import_base.push(Libs, name.to_string());

                app.insert_str(0, &resp)
            }
        }
    }
}