use crate::import_base::ImportBase;
use crate::import_base::ImportType::Mods;
use crate::import_lib::import_lib_bind;

use crate::consts::NEW_LINE;
use std::fs::read_to_string;

pub fn module(app: &mut String, import_base: &mut ImportBase, script: &mut String) {
    while let Some(e) = app.find("import mod:") {
        let mut ci = e + 11;

        while &app[ci..ci + 1] != NEW_LINE {
            ci += 1
        }

        let cloned = app.clone();
        let names = &cloned[e + 11..ci].split(',')
            .collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {
            if import_base.validate(Mods, name.to_string()) {
                let mut module = read_to_string(format!("./{name}.mod.nts"))
                    .unwrap_or_else(|_| panic!("Module {name} not found"));
                import_base.push(Mods, name.to_string());

                import_lib_bind(&mut module, import_base);
                script.insert_str(0, &format!("{module}{NEW_LINE}"));
            }
        }
    }
}
