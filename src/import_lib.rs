use crate::import_base::ImportBase;
use crate::import_base::ImportType::Libs;
use crate::js_lib::libs;

pub fn add_lib(script: &mut String, import_base: &mut ImportBase, lib_name: &str) {
    let string_name = String::from(lib_name);

    if import_base.validate(Libs, string_name.clone()) {
        let resp = libs(lib_name);
        import_base.push(Libs, string_name);
        script.insert_str(0, &resp);
    }
}

pub fn import_lib(app: &mut String, import_base: &mut ImportBase, script: &mut String) {
    while let Some(e) = app.find("import lib:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        let mut pl = String::new();

        for name in names {
            add_lib(&mut pl, import_base, name);
        }

        app.replace_range(e..ci + 1, "");
        script.insert_str(0, &pl);
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
            let string_name = String::from(*name);

            if import_base.validate(Libs, string_name.clone()) {
                let resp = libs(name);
                import_base.push(Libs, string_name);

                app.insert_str(0, &resp)
            }
        }
    }
}
