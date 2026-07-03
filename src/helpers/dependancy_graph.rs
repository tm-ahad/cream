use std::{fs, path::Path, vec::Vec};
use oxc_codegen::{CodegenOptions, CommentOptions};
use crate::component::Component;
use crate::{helpers::javascript::transpile_to_js::transpile_to_js, javascript_lib::libs};
use crate::std_err::{StdErr, ErrType::OSError};
use std::process::exit;
use crate::config::Config;
use crate::consts::BUILD_PATH;

#[derive(Debug)]
pub struct DependencyGraph {
    std: Vec<String>
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {std: vec![]}
    }

    pub fn add_std_lib(&mut self, lib: &str) {
        self.std.push(lib.to_string());
    }

    pub fn install(&mut self, build_path: &str, config: &Config) {
        let std_dir = Path::new(build_path).join(".cream_std");

        fs::create_dir_all(&std_dir)
            .expect("failed to create .cream_std directory");

        for name in &self.std {
            let mut file_path = std_dir.join(name);
            let ext = file_path.extension()
                .unwrap_or_else(|| {
                    StdErr::exec(OSError, &format!("Cant find extension of {name}"));
                    exit(1);
                })
                .to_str()
                .unwrap();

            let lib = libs(name, false);
            let content = match ext {
                "ts" => {
                    transpile_to_js(
                        &lib,
                        name,
                        CodegenOptions {
                            minify: !cfg!(debug_assertions),
                            single_quote: true,
                            comments: CommentOptions::disabled(),
                            ..Default::default()
                        }
                    )
                },
                "cream" => {
                    let mut dep_graph = DependencyGraph::new();
                    let mut comp = Component::new(lib, name.to_string());
                    comp.transpile(config, &mut dep_graph);
                    dep_graph.install(BUILD_PATH, config);

                    comp.out
                },
                _ => lib
            };

            file_path.set_extension("js");
            if file_path.exists() {
                continue;
            }

            fs::write(file_path, content)
                .unwrap_or_else(|e| {
                    StdErr::exec(OSError, &format!("failed to write stdlib file {}: {}", name, e));
                    exit(1);
                });
        }
    }
}

impl Clone for DependencyGraph {
    fn clone(&self) -> Self {
        Self {std: self.std.clone()}
    }
}
