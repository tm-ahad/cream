use std::path::{Path, PathBuf};

pub fn build_path(path: &str) -> String {
    let key = if path == "error" {path} else {&path.replace("/", "_")};
    format!("./build/{key}.js")
}

fn preprocces_path(path: &str) -> String {
    format!(
        "/{}", 
        path.split("/")
            .filter(|a| {!a.is_empty()})
            .collect::<Vec<&str>>()
            .join("/")
    )
}

pub fn std_lib_path(name: &str) -> String {
    let mut path = PathBuf::from(name);
    path.set_extension("js");

    let out_path = Path::new("./.cream_std/")
        .join(path);

    out_path.to_string_lossy().into_owned()
}

pub fn translate_import_src_to_build(path: &str, root: &str) -> String {
    let key = if path == "error" {path} else {&path.replace("/", "_")};
    preprocces_path(&format!("{root}/{key}.js"))
}
