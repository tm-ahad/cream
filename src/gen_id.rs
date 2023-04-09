use rand::{thread_rng, Rng};

pub fn gen_id() -> String { thread_rng().gen::<f64>().to_string() }