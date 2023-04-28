use crate::import_lib::import_lib;
use std::fs::read_to_string;
use crate::import_base::ImportBase;
use crate::import_base::ImportType::Mods;

pub fn module(
    mut app: String,
    import_base: &mut ImportBase,
    js: String,
) -> (String, String) {
    let mut js_ = js;

    while let Some(e) = app.find("import mod:") {
        let mut ci = e + 9;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {
            if import_base.validate(Mods, name.to_string()) {
                let fmt = format!("./src/{name}.mod.nts");

                let module = read_to_string(fmt.clone())
                    .unwrap_or_else(|_| panic!("Module {name} not found"));
                import_base.push(Mods, fmt.clone());

                let t = import_lib(module.clone(), import_base, module.clone(), true).0;
                js_.insert_str(0, &t)
            }
        }
    }

    (app, js_)
}