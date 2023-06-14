use crate::import_base::ImportBase;
use crate::import_base::ImportType::Libs;
use crate::js_lib::libs;

pub fn import_lib(app: &mut String, import_base: &mut ImportBase, js: &mut String, app_started: usize) {
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

                let u = e as i32 - app_started as i32;
                let r: usize = if u > 0 {u as usize}else{0};

                let q = ci as i32 - app_started as i32;
                let p: usize = if q > 0 {q as usize}else{0};

                js.replace_range(r..p+1, &resp);
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
            let string_name = String::from(*name);

            if import_base.validate(Libs, string_name.clone()) {
                let resp = libs(name);
                import_base.push(Libs, string_name);

                app.insert_str(0, &resp)
            }
        }
    }
}