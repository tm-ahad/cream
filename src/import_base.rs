pub enum ImportType {
    Mods,
    Libs,
    Scripts,
}

pub struct ImportBase {
    mods: Vec<String>,
    libs: Vec<String>,
    scripts: Vec<String>,
}

impl ImportBase {
    pub fn new() -> ImportBase {
        ImportBase {
            mods: Vec::new(),
            libs: Vec::new(),
            scripts: Vec::new(),
        }
    }

    pub fn validate(&self, tp: ImportType, v: String) -> bool {
        !match tp {
            ImportType::Mods => &self.mods,
            ImportType::Libs => &self.libs,
            ImportType::Scripts => &self.scripts,
        }
        .contains(&v)
    }

    pub fn push(&mut self, tp: ImportType, v: String) {
        match tp {
            ImportType::Mods => &mut self.mods,
            ImportType::Libs => &mut self.libs,
            ImportType::Scripts => &mut self.scripts,
        }
        .push(v)
    }
}
