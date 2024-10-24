use crate::consts::NEW_LINE;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use std::fs::read_to_string;

pub fn import_template(app: &mut String, f_name: &str, html: &mut String) {
    let pat = "import template:";

    while let Some(i) = app.find(pat) {
        let start = i+pat.len();
        let ci = read_until(app, start, NEW_LINE, f_name, ComponentPart::Unknown);

        let cloned = app.clone();

        let names = &cloned[start..ci].split(',').collect::<Vec<&str>>();
        app.replace_range(i..ci + 1, "");

        for name in names {
            let fmt = format!("./{name}");
            let resp = read_to_string(&fmt)
                .unwrap_or_else(|_| panic!("Script '{fmt}' not found"));

            html.insert_str(0, &resp)
        }
    }
}