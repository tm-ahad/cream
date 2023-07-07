pub struct BracePool(Vec<bool>);

impl BracePool {
    pub fn new() -> BracePool {
        BracePool(Vec::new())
    }

    pub fn push(&mut self, c: char) -> bool {
        match self.0.last() {
            Some(a) => if *a {
                let b = c == '}';

                return if b && self.0.starts_with(&[true]) {
                    self.0.remove(self.0.len() - 1);
                    false
                } else if b  {
                    true
                } else {
                    self.0.push(c == '{');
                    false
                }

            },
            None => {
                self.0.push(c == '{');
                return false;
            }
        }

        todo!()
    }
}