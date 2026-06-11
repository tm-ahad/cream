
pub enum ComponentPart {Unknown}

impl ComponentPart {
    pub fn to_raw_string(&self) -> String {
        String::from("unknown")
    }
}
