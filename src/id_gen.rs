use std::env::{set_var, var};

pub struct IdGen;

impl IdGen {
    pub fn init() {
        match var(":nID") {
            Ok(_) => panic!(":nID found on env var"),
            Err(_) => set_var(":nID", "0")
        }
    }

    pub fn get_and_update() -> String {
        let count = var(":nID")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        set_var(":nID", (count+1).to_string());
        format!(":n{count}")
    }
}
