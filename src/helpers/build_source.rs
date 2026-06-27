pub fn build_path(path: &str) -> String {
    let key = if path == "error" {path} else {&path.replace("/", "_")};
    format!("./build/{key}.js")
}

pub fn translate_import_src_to_build(path: &str) -> String {
    let key = if path == "error" {path} else {&path.replace("/", "_")};
    format!("./{key}.js")
}
