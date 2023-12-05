pub fn javascript_function_call(name: &str, params: Vec<String>) -> String {
    let mut pr = String::new();

    for s in params {
        pr.push_str(&s);
        pr.push(',')
    }

    format!("{name}({pr})")
}
