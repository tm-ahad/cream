use crate::std_err::{ErrType::PackageError, StdErr};
use ureq::{get, Response};

pub fn import_npm(app: &mut String, js: &mut String) {
    while let Some(i) = app.find("import npm:") {
        let mut end = i + 11;

        while &app[end..end + 1] != "\n" {
            end += 1;
        }

        let url = format!(
            "http://cdn.jsdelivr.net/npm/{}/lib/index.umd.js",
            &app[i + 11..end]
        );

        let resp = get(&url).call().unwrap_or_else(|e| {
            StdErr::exec(PackageError, &e.to_string());
            Response::new(404, "PackageError", "").unwrap()
        });

        if resp.status() == 200 {
            let script = resp.into_string().unwrap_or_else(|e| panic!("{e}"));

            js.insert_str(0, &script)
        }

        app.replace_range(i..end + 1, "")
    }
}
