use crate::import_base::ImportBase;
use crate::import_base::ImportType::Scripts;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::consts::NEW_LINE;
use std::fs::read_to_string;

pub fn import_script(
    app: &mut String,
    import_base: &mut ImportBase,
    script: &mut String,
    f_name: &str
) {
    while let Some(e) = app.find("import script:") {
        let ci = read_until(&app, e+13, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();

        let names = &cloned[e + 11..ci].split(',').collect::<Vec<&str>>();
        app.replace_range(e..ci + 1, "");

        for name in names {
            if import_base.validate(Scripts, name.to_string()) {
                let fmt = format!("./src/{name}");
                let resp = read_to_string(&fmt).unwrap_or_else(|_| panic!("Script {name} not found"));

                import_base.push(Scripts, fmt);
                script.insert_str(0, &resp)
            }
        }
    }
}
