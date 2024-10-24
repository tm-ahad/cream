use crate::consts::{NEW_LINE, NIL};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::import_base::ImportBase;
use crate::import_base::ImportType::Libs;
use crate::javascript_lib::libs;

pub fn add_lib(script: &mut String, import_base: &mut ImportBase, lib_name: &str) {
    let string_name = String::from(lib_name);

    if import_base.validate(Libs, string_name.clone()) {
        let resp = libs(lib_name, false);
        import_base.push(Libs, string_name);
        script.insert_str(0, &resp);
    }
}

pub fn import_lib(
    app: &mut String,
    import_base: &mut ImportBase,
    script: &mut String,
    f_name: &str
) {
    while let Some(e) = app.find("import lib:") {
        let ci = read_until(app, e+11, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        let mut pl = String::new();

        for name in names {
            add_lib(&mut pl, import_base, name);
        }

        app.replace_range(e..ci + 1, NIL);
        script.insert_str(0, &pl);
    }
}

pub fn import_lib_bind(app: &mut String, import_base: &mut ImportBase, f_name: &str) {
    while let Some(e) = app.find("import lib:") {
        let ci = read_until(app, e+11, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, NIL);

        for name in names {
            let string_name = String::from(*name);

            if import_base.validate(Libs, string_name.clone()) {
                let resp = libs(name, false);
                import_base.push(Libs, string_name);

                app.insert_str(0, &resp)
            }
        }
    }
}
