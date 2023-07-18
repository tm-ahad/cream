pub fn expect_some<T>(val: Option<T>, expectation: &str) -> T {
    match val {
        Some(val) => val,
        None => panic!("{expectation} is expected"),
    }
}
