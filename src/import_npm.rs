use crate::std_err::{ErrType::PackageError, StdErr};
use crate::consts::{NEW_LINE, NEW_LINE_CHAR, NIL};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use tinyget::get;

pub fn import_npm(app: &mut String, script: &mut String, f_name: &str) {
    while let Some(i) = app.find("import npm:") {
        let end = read_until(&app, i+11, NEW_LINE, f_name, ComponentPart::Unknown);
        let pkg = &app[i + 11..end];

        let url = format!(
            "http://cdn.jsdelivr.net/npm/{}/lib/index.umd.js",
            pkg
        );

        let resp = get(&url).send().unwrap_or_else(|e| {
            StdErr::exec(PackageError, &e.to_string());
            todo!()
        });

        if resp.status_code == 200 {
            let mut pack = resp.as_str().unwrap_or_else(|e| panic!("{e}")).to_string();
            pack.push(NEW_LINE_CHAR);

            script.insert_str(0, &pack)
        } else {
            StdErr::exec(PackageError, &format!("Package {pkg} not found"));
        }

        app.replace_range(i..end + 1, NIL)
    }
}
