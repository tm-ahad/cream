use crate::consts::{
    BACK_TICK, BACK_TICK_AS_STR, DOUBLE_QUOTE, DOUBLE_QUOTE_AS_STR, SINGLE_QUOTE,
    SINGLE_QUOTE_AS_STR,
};
use crate::pass::pass;
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
    ch: char,
}

impl UpdateIBIS {
    pub fn new(init_ch: char, curr: bool) -> Self {
        Self { ch: init_ch, curr }
    }

    pub fn update(&mut self, s: &str) -> bool {
        let mut b = self.curr;

        match (self.ch, s) {
            (_, DOUBLE_QUOTE_AS_STR | SINGLE_QUOTE_AS_STR | BACK_TICK_AS_STR)
            | (DOUBLE_QUOTE | SINGLE_QUOTE | BACK_TICK, _) => b = !b,
            _ => pass(),
        }

        self.curr = b;
        b
    }
}
