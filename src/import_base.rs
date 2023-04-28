pub enum ImportType {
    Mods,
    Libs,
    Scripts
}

#[derive(Debug)]
pub struct ImportBase {
    pub mods: Vec<String>,
    pub libs: Vec<String>,
    pub scripts: Vec<String>
}

impl ImportBase {
    pub fn new() -> ImportBase {
        Self {
            mods: Vec::new(),
            libs: Vec::new(),
            scripts: Vec::new()
        }
    }

    pub fn validate(&self, tp: ImportType, v: String) -> bool {
        !match tp {
            ImportType::Mods => &self.mods,
            ImportType::Libs => &self.libs,
            ImportType::Scripts => &self.scripts
        }.contains(&v)
    }

    pub fn push(&mut self, tp: ImportType, v: String) {
        match tp {
            ImportType::Mods => &mut self.mods,
            ImportType::Libs => &mut self.libs,
            ImportType::Scripts => &mut self.scripts
        }.push(v)
    }
}