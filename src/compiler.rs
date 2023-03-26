use crate::collect_gen::collect_gen;
use crate::component::Component;
use crate::component::{component, parse};
use crate::scope::scope;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;
use crate::template::template;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::{self, read_to_string};
use crate::at_html::at_html;

pub fn compile(name: &String, mut state: _StateBase) {
    let mut app = read_to_string(format!("./{}/src/app.js", name)).expect("app.js not found");
    let mut imports: Vec<Component> = vec![];
    let mut names: Vec<String> = vec![];
    let mut fail = String::new();

    let mut router_hash: HashMap<String, String> = HashMap::new();

    let main_app = collect_gen(app.clone(), "app{".to_string(), 0, "}");

    let mut js = String::new();
    let split = main_app.split("\n");

    while app.contains("import component") {
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

                names.push(app[e + 16..namei].trim().to_string());
                imports.push(component(name, fnm.to_string(), cn.trim().to_string(),
                                       &mut state));


                app.replace_range(e..ci+1, "")
            }
        }
    }


    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else {
            break;
        }
    }

    let mut comp_html = collect_gen(main_app, "<html>".to_string(), 0, "</html>");

    while app.contains("import lib:") {
        match app.find("import lib:") {
            None => {}
            Some(e) => {
                let mut ci = e + 9;

                while &app[ci..ci + 1] != "\n" {
                    ci += 1
                }

                comp_html = format!("{comp_html}\n<script type=\"modules\" src=\"./lib/{}\"></script>",
                    &app[e+9..ci+1]);

                app.replace_range(e..ci+1, "")
            }
        }
    }

    while js.find("Router") != None {
        match js.find("Router") {
            Some(a) => {
                let start = a + 7;
                let mut idx = 0;
                let mut val = 0;

                while &js[idx..idx + 1] != "=" && &js[idx..idx + 1] != "}" {
                    idx += 1
                }

                while &js[val..val + 1] != "}" {
                    val += 1
                }

                match serde_json::from_str::<Value>(&js[idx + 1..val + 1]) {
                    Ok(_) => {
                        router_hash.insert(
                            js[start..idx].trim().to_string(),
                            js[idx + 1..val + 1].to_string(),
                        );
                        js.replace_range(start - 7..val + 1, "");
                    }
                    Err(e) => {
                        let err = StdErr::new(SyntaxError, "Invalid Object Router");

                        err.exec();
                        panic!("{e}")
                    }
                }
            }
            _ => {}
        }
    }

    let ht = at_html(comp_html.clone(), js.clone());

    comp_html = ht.0;
    js = ht.1;

    let caught = template(comp_html, js.clone());

    js = caught.1;
    comp_html = caught.0;

    js = _state(js.clone(), &mut state);
    let scoope = scope(comp_html.clone(), js.clone(), &mut state);

    js = scoope.0;
    comp_html = scoope.1;

    js = js.replace(".single()", "");

    match comp_html.find("<Router route=") {
        None => {}
        Some(a) => {
            if &comp_html[a + 14..a + 15] == "{" {
                let mut idx = a;

                while &comp_html[idx..idx + 1] != "}" {
                    idx += 1
                }

                let json = &comp_html[a + 14..idx + 1];

                match serde_json::from_str::<Value>(json) {
                    Ok(a) => {
                        let obj = a.as_object().unwrap();

                        let mut map = Map::new();

                        for (key, val) in obj {
                            let s = val.as_str().unwrap();
                            map.insert(
                                key.clone(),
                                Value::String(parse(&component(
                                    name,
                                    s.to_string(),
                                    "Render".to_string(),
                                    &mut state
                                ))),
                            );
                        }

                        js = format!(
                            "{}\n{}",
                            js,
                            format!(
                                "var Route = {}",
                                serde_json::to_string::<Value>(&Value::Object(map)).unwrap()
                            )
                        );
                    }
                    Err(_) => panic!("Can't evern parse njsion in ohio"),
                }

                js = format!(
                    "{js}\n{}",
                    "\
    function main() {
        for (let k in Route) {
            if (window.location.pathname == k) {
                document.getElementById(\"app\").innerHTML = Route[k]
                window.history.pushState({}, \"\", k)
            }
        }
    }

    main()
                "
                );

                comp_html.replace_range(a..idx + 2, "")
            } else {
                let mut idx = a + 14;

                while &comp_html[idx..idx + 1] != "/" {
                    idx += 1
                }

                let name_ = comp_html[a + 14..idx].trim();
                let binding: Value;

                match router_hash.get(name_) {
                    Some(a) => {
                        binding = serde_json::from_str::<Value>(a).unwrap();

                        let obj = binding.as_object().unwrap();

                        let mut map = Map::new();

                        for (key, val) in obj {
                            let s = val.as_str().unwrap();
                            map.insert(
                                key.clone(),
                                Value::String(parse(&component(
                                    name,
                                    s.to_string(),
                                    "Render".to_string(),
                                    &mut state
                                ))),
                            );
                        }

                        js = format!(
                            "{}\n{}",
                            js,
                            format!(
                                "var Route = {}",
                                serde_json::to_string::<Value>(&Value::Object(map)).unwrap()
                            )
                        );
                    }
                    _ => {}
                }

                js = format!(
                    "{js}\n{}",
                    "\
    function main() {
        for (let k in Route) {
            if (window.location.pathname == k) {
                document.body.innerHTML = Route[k]
                window.history.pushState({}, \"\", k)
            }
        }
    }

    main()
                "
                );

                comp_html.replace_range(a..idx + 2, "");
            }
        }
    }

    for n in names {
        fail = format!("<{}/>", n);
        let m = fail.as_str();
        if comp_html.contains(m.clone()) {
            for i in &imports {
                if i.name == n {
                    match comp_html.find(m.clone()) {
                        Some(e) => {
                            comp_html.replace_range(e..m.len() + 1, &*i.html);
                            js = format!("{js}\n{}", i.js);
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    fs::write(
        format!("./{}/build/index.html", name),
        format!(
            "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Document</title>
</head>
<body>
    {comp_html}
    <script type=\"module\">
    {js}
    </script>
</body>
</html>
",
        ),
    )
    .expect("File not found or writing not supported");
}
