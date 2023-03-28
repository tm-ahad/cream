use crate::collect_gen::collect_gen;
use std::fs::read_to_string;
use rusty_v8::{ContextScope, HandleScope, Local, Script};
use crate::at_html::at_html;
use crate::scope::_scope;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::template::template;

#[derive(Debug)]
pub struct Component {
    pub js: String,
    pub html: String,
    pub name: String,
}

pub fn component(p_name: &String, f_name: String, c_name: String, script: Local<Script>,
    scope: &mut ContextScope<HandleScope>,
    st: &mut _StateBase) -> Component {

    let path = format!("./{}/src/{}", p_name, f_name).replace("\"", "");

    let app = read_to_string(path).expect("file not found");
    let mut _imports: Vec<Component> = vec![];
    let mut _names: Vec<String> = vec![];

    let mut macher = c_name.clone();
    macher.push_str("{");

    let main_app = collect_gen(app.clone(), macher, 0, "}");

    let mut js = String::new();
    let split = main_app.split("\n").collect::<Vec<&str>>();

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else {
            break;
        }
    }

    let mut html = collect_gen(main_app, "<html>".to_string(), 0, "<html/>");
    let caught = template(html, js.clone(), scope, st);

    js = caught.1;
    html = caught.0;

    let ht = at_html(html.clone(), js.clone());

    html = ht.0;
    js = ht.1;

    js = _state(js.clone(), st);
    let catch = _scope(html.clone(), js.clone(), st);

    js = catch.0;
    html = catch.1;

    js = js.replace(".single()", "");

    match app.find("import component") {
        None => {}
        Some(e) => {
            let mut namei = e + 17;
            let mut ci = e + 30;
            let mut cn: String = String::new();
            let mut fnm: String = String::new();

            while &app[namei..namei + 4] != "from" {
                cn.push(app.chars().nth(namei).unwrap());
                namei += 1;
            }

            while &app[ci..ci + 1] != "\n" {
                fnm.push(app.chars().nth(ci).unwrap());
                ci += 1
            }

            _names.push(app[e + 16..namei].trim().to_string());
            _imports.push(component(p_name, fnm.to_string(), cn.trim().to_string(),
                script, scope, st));
        }
    }

    let mut fail = String::new();

    for n in _names {
        fail = format!("<{}/>", n);
        let m = fail.as_str();
        if html.contains(m.clone()) {
            for i in &_imports {
                if i.name == n {
                    match html.find(m.clone()) {
                        Some(e) => {
                            html.replace_range(e..m.len() + 1, &*i.html);
                            js = format!("{js}\n{}", i.js)
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    return Component {
        js,
        html,
        name: c_name,
    };
}

pub fn parse(s: &Component) -> String {
    let result = format!(
        "
{}
<script>
{}
<script>
    ",
        s.html, s.js
    );

    return result;
}
