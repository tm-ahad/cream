pub enum TemplateType {
    Dynamic,
    Static,
}

impl TemplateType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "dyn" | "dynamic" => Self::Dynamic,
            "" => Self::Static,
            _ => panic!("Invalid template type"),
        }
    }

    pub fn is_dynamic(&self) -> bool {
        match *self {
            TemplateType::Static => false,
            TemplateType::Dynamic => true,
        }
    }
}
