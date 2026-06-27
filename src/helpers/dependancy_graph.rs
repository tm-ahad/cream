use std::{fs, path::{Path, PathBuf}, vec::Vec};
use oxc_codegen::{CodegenOptions, CommentOptions};
use crate::{helpers::javascript::transpile_to_js::transpile_to_js, javascript_lib::libs};

#[derive(Debug)]
pub struct DependancyGraph {
    std: Vec<String>
}

impl DependancyGraph {
    pub fn new() -> Self {
        Self {std: vec![]}
    }

    pub fn add_std_lib(&mut self, lib: &str) {
        self.std.push(lib.to_string());
    }

    pub fn install(&self, build_path: &str) {
        let std_dir = Path::new(build_path).join(".cream_std");

        fs::create_dir_all(&std_dir)
            .expect("failed to create .cream_std directory");

        for name in &self.std {
            let file_path = std_dir.join(name);
            let content = transpile_to_js(
                &libs(name, false),
                name,
                CodegenOptions {
                    minify: !cfg!(debug_assertions),
                    single_quote: true,
                    comments: CommentOptions::disabled(),
                    ..Default::default()
                }
            );

            let mut build_path = PathBuf::from(file_path);
            build_path.set_extension("js");
            
            fs::write(&build_path, content)
                .unwrap_or_else(|e| {
                    panic!("failed to write stdlib file {}: {}", name, e);
                });
        }
    }
}

impl Clone for DependancyGraph {
    fn clone(&self) -> Self {
        Self {std: self.std.clone()}
    }
}
