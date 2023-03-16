use crate::collect_gen::collect_gen;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Component {
    pub js: String,
    pub html: String,
    pub name: String,
}

pub fn component(p_name: &String, f_name: String, c_name: String) -> Component {
    let path = format!("./{}/src/{}", p_name, f_name)
        .replace("\"", "");

    let app = read_to_string(path).expect("file not found");
    let mut imports: Vec<Component> = vec![];
    let mut names: Vec<String> = vec![];

    let mut macher = c_name.clone();
    macher.push_str(" {");

    let main_app = collect_gen(app.clone(), macher, 0, "}");

    let mut js = String::new();
    let split = main_app.split("\n").collect::<Vec<&str>>();
    #[allow(unused_assignments)]
    let mut fail = String::new();

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else {
            break;
        }
    }

    let mut html = collect_gen(main_app, "<html>".to_string(), 0, "<html/>");

    match app.find("import component") {
        None => {}
        Some(e) => {
            let mut namei = e + 17;
            let mut ci = e + 29;
            let mut cn: String = String::new();
            let mut fnm: String = String::new();

            while &app[namei..namei + 4] != "from" {
                cn.push(app.chars().nth(namei).unwrap());
                namei += 1;
            }

            while &app[ci..ci+1] != "\n" {
                fnm.push(app.chars().nth(ci).unwrap());
                ci += 1
            }

            names.push(app[e + 16..namei].trim().to_string());
            imports.push(component(p_name, fnm.to_string(), cn.trim().to_string()));
        }
    }

    for n in names {
        fail = format!("<{}/>", n);
        let m = fail.as_str();

        if html.contains(m.clone()) {
            for i in &imports {
                if i.name == n {

                    match html.find(m.clone()) {
                        Some(e) => html.replace_range(e..m.len() + 1, &*parse(i)),
                        _ => {}
                    }
                }
            }
        }
    }
    println!("{}", html);

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
