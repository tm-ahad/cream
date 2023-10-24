pub struct ComponentMarkUp {
    pub dynamic: String,
    pub stat: String, // STATIC -> STAT
}

impl ComponentMarkUp {
    pub const EMPTY: ComponentMarkUp = Self {
        dynamic: String::new(),
        stat: String::new(),
    };

    pub fn new(dynamic: String, stat: String) -> Self {
        ComponentMarkUp { dynamic, stat }
    }
}

impl Clone for ComponentMarkUp {
    fn clone(&self) -> Self {
        ComponentMarkUp {
            dynamic: self.dynamic.clone(),
            stat: self.stat.clone(),
        }
    }
}
