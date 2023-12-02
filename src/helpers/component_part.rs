
pub enum ComponentPart {
    Template,
    Unknown,
    Script
}

impl ComponentPart {
    pub fn to_raw_string(&self) -> String {
        String::from(match self {
            ComponentPart::Template => "template",
            ComponentPart::Unknown => "unknown",
            ComponentPart::Script => "script",
        })
    }
}
