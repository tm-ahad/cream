#[derive(Clone)]
pub struct Mp(pub String);

impl Mp {
    pub fn new(s: String, _i: usize, _ext: Option<usize>) -> Mp {
        Mp(s)
    }
    pub fn mp_val(self) -> String {
        self.0
    }
}
