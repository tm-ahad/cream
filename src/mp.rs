#[derive(Clone)]
pub struct Mp(pub String, pub usize, Option<usize>);

impl Mp {
    pub fn new(s: String, i: usize, ext: Option<usize>) -> Mp {
        Mp(s, i, ext)
    }

    pub fn mp(&self) -> &str {
        &self.0
    }
    pub fn mp_val(self) -> String {
        self.0
    }
    pub fn ext(&self) -> &Option<usize> {
        &self.2
    }
    pub fn index(&self) -> usize {
        self.1
    }
}
