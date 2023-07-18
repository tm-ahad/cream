#[derive(Clone)]
pub struct Mp(pub String, pub usize);

impl Mp {
    pub fn new(s: String, i: usize) -> Mp {
        Mp(s, i)
    }

    pub fn mp(&self) -> &String {
        &self.0
    }

    pub fn mp_len(self) -> usize {
        self.0.len()
    }
    pub fn mp_val(self) -> String {
        self.0
    }

    pub fn index(&self) -> usize {
        self.1
    }
}
