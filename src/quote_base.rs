#[derive(PartialEq)]
pub enum Quote {
    Single,
    Double,
    Backtick,
}

impl Quote {
    pub fn from_char(c: char) -> Quote {
        match c {
            '\'' => Quote::Single,
            '"' => Quote::Double,
            '`' => Quote::Backtick,
            _ => panic!("Invalid quote"),
        }
    }
}
pub struct QuotePool(Vec<Quote>);

impl QuotePool {
    pub fn new() -> QuotePool {
        QuotePool(Vec::new())
    }

    pub fn push(&mut self, q: Quote) {
        return match self.0.last() {
            Some(a) => {
                if a.eq(&q) {
                    let len = self.0.len();
                    self.0.remove(len - 1);
                } else {
                    self.0.push(q)
                }
            }
            None => self.0.push(q),
        };
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_empty()
    }
}
