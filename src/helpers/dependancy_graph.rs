use std::{fs, marker::PhantomData, path::Path, vec::Vec};

use crate::javascript_lib::libs;

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
            let content = libs(name, false);

            fs::write(&file_path, content)
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
