use std::fs::read_to_string;
use crate::collect_gen::collect_gen;

#[derive(Debug)]
pub struct Component {
    pub js: String,
    pub html: String,
    pub name: String
}

pub fn component(p_name: &String, f_name: String, c_name: String) -> Component {
    let app = read_to_string(format!("./{}/src/{}", p_name, f_name))
        .expect("file not found");

    let mut macher = c_name.clone();
    macher.push_str(" {");

    let main_app = collect_gen(app.clone(), macher,
                               0, "}");

    let mut js = String::new();
    let split = main_app.split("\n");

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else { break }
    }

    let html = collect_gen(main_app, "<html>".to_string(),
                           0, "<html/>");

    return Component{
        js,
        html,
        name: c_name
    }
}