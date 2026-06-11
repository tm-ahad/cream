use crate::import_base::ImportBase;
use crate::import_base::ImportType::Mods;
use crate::consts::NEW_LINE;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;

pub fn module(
    app: &mut String,
    import_base: &mut ImportBase,
    _script: &mut String,
    f_name: &str,
) {
    while let Some(e) = app.find("import mod:") {
        let ci = read_until(app, e+11, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();
        let names = &cloned[e + 11..ci]
            .split(',')
            .collect::<Vec<&str>>();

        app.replace_range(e..ci + 1, "");

        for name in names {
            import_base.push(Mods, name.to_string());
        }
    }
}
