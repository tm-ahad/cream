pub struct BracePool(pub Vec<u32>);

impl BracePool {
    pub fn new() -> BracePool {
        BracePool(Vec::new())
    }

    pub fn push(&mut self, c: char) -> bool {
        match self.0.last_mut() {
            Some(a) => {
                let bit = (*a & 1) == 1;
                if bit {
                    if c == '{' {
                        *a = (*a << 1) | 1;
                        false
                    } else {
                        *a >>= 1;

                        if *a == 0 {
                            self.0.remove(self.0.len()-1);
                        }
                        self.0.is_empty()
                    }
                } else {
                    panic!("It isn't possible!");
                }
            }
            None => {
                if c == '}' {
                    panic!("Syntax Error!");
                }
                let new_value = if c == '{' { 1 } else { 0 };
                self.0.push(new_value);
                false
            }
        }
    }
}
