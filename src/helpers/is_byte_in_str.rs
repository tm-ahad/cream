use crate::consts::{BACK_TICK, DOUBLE_QUOTE, NUL, SINGLE_QUOTE};
use crate::quote_base::{Quote, QuotePool};
use crate::pass::pass;

pub fn is_byte_in_str(index: usize, str: &str) -> bool {
    let pattern = |&c: &char| {
        c == DOUBLE_QUOTE || c == SINGLE_QUOTE || c == BACK_TICK
    };

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
    pub curr: bool,
    dx: bool,
    st: char,
}

impl UpdateIBIS {
    pub fn new(curr: bool) -> Self {
        Self { curr, st: NUL, dx: false }
    }

    pub fn update(&mut self, s: &str) -> bool {
        let fc = s.chars().next().unwrap();

        self.curr ^= self.dx;
        self.dx = false;

        match (self.st, fc) {
            (fc_, _) if fc_ == fc => {
                self.curr ^= true
            },
            (NUL, SINGLE_QUOTE | DOUBLE_QUOTE | BACK_TICK) => {
                self.dx = true;
                self.st = fc
            },
            (_, _) => pass()
        }

        return self.curr
    }
}