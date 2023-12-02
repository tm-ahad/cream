use crate::import_base::ImportBase;
use crate::import_base::ImportType::Mods;
use crate::import_lib::import_lib_bind;

use crate::consts::NEW_LINE;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use std::fs::read_to_string;

pub fn module(
    app: &mut String,
    import_base: &mut ImportBase,
    script: &mut String,
    f_name: &str,
) {
    while let Some(e) = app.find("import mod:") {
        let ci = read_until(&app, e+11, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();
        let names = &cloned[e + 11..ci]
            .split(',')
            .collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {
            if import_base.validate(Mods, name.to_string()) {
                let mut module = read_to_string(format!("./{name}.mod.cts"))
                    .unwrap_or_else(|_| panic!("Module {name}.mod.cts not found"));
                import_base.push(Mods, name.to_string());

                import_lib_bind(&mut module, import_base, f_name);
                script.insert_str(0, &format!("{module}{NEW_LINE}"));
            }
        }
    }
}
