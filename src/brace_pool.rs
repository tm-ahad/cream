pub struct BracePool(pub Vec<bool>);

impl BracePool {
    pub fn new() -> BracePool {
        BracePool(Vec::new())
    }

    pub fn push(&mut self, c: char) -> bool {
        return match self.0.last() {
            Some(a) => {
                if *a {
                    let b = c == '}';

                    return if b {
                        let len = self.0.len();
                        self.0.remove(len - 1);

                        return if self.0.len() == 0 {
                            true
                        } else {
                            false
                        }
                    } else {
                        self.0.push(true);
                        false
                    };
                } else {
                    panic!("It isn't possible!")
                }
            }
            None => {
                if c == '}' {
                    panic!("Syntax Error!")
                }

                self.0.push(c == '{');
                false
            }
        };
    }
}