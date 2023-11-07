use crate::consts::{BACK_TICK, DOUBLE_QUOTE, NUL, SINGLE_QUOTE};
use crate::quote_base::{Quote, QuotePool};

pub fn is_byte_in_str(index: usize, str: &str) -> bool {
    let pattern = |&c: &char| c == '"' || c == '\'' || c == '`';

    let start = &str[..index];
    let end = &str[index + 1..];

    let mut sqp = QuotePool::new();
    let mut eqp = QuotePool::new();

    start.chars().filter(pattern).for_each(|c| {
        sqp.push(Quote::from_char(c));
    });

    end.chars().filter(pattern).for_each(|c| {
        eqp.push(Quote::from_char(c));
    });

    !sqp.is_valid() && !eqp.is_valid()
}

pub struct UpdateIBIS {
    curr: bool,
    xw: bool,
    st: char,
}

impl UpdateIBIS {
    pub fn new(curr: bool) -> Self {
        Self { curr, xw: false, st: NUL }
    }

    pub fn update(&mut self, s: &str) -> bool {
        let fc = s.chars().next().unwrap();

        match (self.st, fc) {
            (fc_, _) if fc_ == fc => {
                self.curr ^= true
            },
            (NUL, SINGLE_QUOTE | DOUBLE_QUOTE | BACK_TICK) => {
                self.curr = true;
                self.st = fc
            }
            (_, _) => {
                self.curr ^= self.xw;
                self.xw = false;
            }
        }

        return self.curr
    }
}
