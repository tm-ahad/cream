pub struct BracePool(Vec<bool>);

impl BracePool {
    pub fn new() -> BracePool {
        BracePool(Vec::new())
    }

    pub fn push(&mut self, c: char) -> bool {
        match self.0.last() {
            Some(a) => {
                if *a {
                    let b = c == '}';

                    return if b {
                        let len = self.0.len();
                        self.0.remove(len - 1);

                        if len > 1 {
                            self.0.remove(0);
                        }
                        false
                    } else {
                        panic!("Syntax error!")
                    };
                }
            }
            None => {
                self.0.push(c == '{');
                return c == '}';
            }
        }

        panic!("Syntax error!")
    }
}