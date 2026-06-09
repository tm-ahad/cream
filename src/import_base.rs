use std::{collections::HashSet, fs::read_to_string};
use crate::javascript_lib::libs;

pub enum ImportType {
    Mods,
    Libs,
    Scripts,
}

pub struct ImportBase {
    mods: HashSet<String>,
    libs: HashSet<String>,
    scripts: HashSet<String>,
}

impl ImportBase {
    pub fn new() -> ImportBase {
        ImportBase {
            mods: HashSet::new(),
            libs: HashSet::new(),
            scripts: HashSet::new(),
        }
    }

    pub fn push(&mut self, tp: ImportType, v: String) {
        match tp {
            ImportType::Mods => &mut self.mods,
            ImportType::Libs => &mut self.libs,
            ImportType::Scripts => &mut self.scripts,
        }
        .insert(v);
    }

    pub fn patch(&mut self, script: &mut String) {
        for lib_name in &self.libs {
            let resp = libs(&lib_name, false);
            script.insert_str(0, &resp);
        }

        for mod_name in &self.mods {
            let module = read_to_string(format!("./{mod_name}.mod.cts"))
                .unwrap_or_else(|_| panic!("Module {mod_name}.mod.cts not found"));
            script.insert_str(0, &format!(";{module};"));
        }

        for script_name in &self.scripts {
            let fmt = format!("./{script_name}");
            let resp = read_to_string(&fmt)
                .unwrap_or_else(|_| panic!("Script '{fmt}' not found"));

            script.insert_str(0, &resp)
        }
    }
}

impl Clone for ImportBase {
    fn clone(&self) -> Self {
        ImportBase { 
            mods: self.mods.clone(), 
            libs: self.libs.clone(), 
            scripts: self.scripts.clone()
        }
    }
}