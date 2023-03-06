use std::fs::read_to_string;
use crate::collect_gen::collect_gen;

pub fn compile(name: &String) {
    let app = read_to_string(format!("./{}/src/app.js", name))
        .expect("app.js not found");

    let main_app = collect_gen(app.clone(), "app {".to_string(),
        0, "}");
    let html = collect_gen(main_app, "<html>".to_string(),
                           0, "<html/>");

    println!("{}", html);
}