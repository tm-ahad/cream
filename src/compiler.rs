use crate::at_html::at_html;
use crate::collect_gen::collect_gen;
use crate::component::Component;
use crate::component::{component, parse};
use crate::scope::_scope;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::template::template;
use crate::pass::pass;
use rusty_v8 as v8;
use serde_json::{Map, Value};
use std::fs::{self, read_to_string};
use rusty_v8::json::stringify;
use rusty_v8::Script;

pub fn compile(name: &String, mut state: _StateBase) {
    let mut app = read_to_string(format!("./{}/src/app.js", name)).expect("app.js not found");
    let mut imports: Vec<Component> = vec![];
    let mut names: Vec<String> = vec![];
    let mut fail = String::new();

    let main_app = collect_gen(app.clone(), "app{".to_string(), 0, "}");

    let mut js = String::new();
    let split = main_app.split("\n");

    for s in split {
        if s != "<html>" {
            js = format!("{}\n{}", js, s);
        } else {
            break;
        }
    }
    
    
    let mut comp_html = format!(
        "{}\n",
        collect_gen(main_app.clone(), "<html>".to_string(), 0, "</html>")
    );

    while app.contains("import lib:") {
        match app.find("import lib:") {
            None => {}
            Some(e) => {
                let mut ci = e + 9;

                while &app[ci..ci + 1] != "\n" {
                    ci += 1
                }

                comp_html = format!(
                    "{comp_html}\n<script type=\"modules\" src=\"./lib/{}\"></script>",
                    &app[e + 9..ci + 1]
                );

                app.replace_range(e..ci + 1, "")
            }
        }
    }

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let ben = &*js.replace(".cam()", "");
    
    let code = v8::String::new(scope, ben)
        .unwrap();

    let mut script = v8::Script::compile(scope, code, None).unwrap();

    let _ = script.run(scope).unwrap();

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
                imports.push(component(
                    name,
                    fnm.to_string(),
                    cn.trim().to_string(),
                    script,
                    scope,
                    &mut state,
                ));

                app.replace_range(e..ci + 1, "")
            }
        }
    }
    
    let ht = at_html(comp_html.clone(), js.clone(), scope, &mut state);

    comp_html = ht.0;
    js = ht.1;

    js = _state(js.clone(), &mut state, scope);
    
    let scoope = _scope(comp_html.clone(), js.clone(), &mut state);

    js = scoope.0;
    comp_html = scoope.1;

    let caught = template(comp_html, js.clone(), scope, &mut state);

    js = caught.1;
    comp_html = caught.0;

    js = js.replace(".single()", "");
    js = ben.to_string();

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
                                    script,
                                    scope,
                                    &mut state,
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
                    Err(_) => panic!("Can't even parse json in ohio"),
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

                let v8_str = v8::String::new(scope, name_)
                    .unwrap();

                script = Script::compile(scope, v8_str, None)
                    .unwrap();

                let res = script
                    .run(scope)
                    .unwrap();

                let router = stringify(scope, res)
                    .unwrap()
                    .to_rust_string_lossy(scope);

                binding = serde_json::from_str::<Value>(&*router).unwrap();

                let obj = binding.as_object().unwrap();

                let mut map = Map::new();

                for (key, val) in obj {
                    let s = val.as_str().unwrap();
                    let _ = map.insert(
                       key.clone(),
                       Value::String(parse(&component(
                                name,
                                s.to_string(),
                                "Render".to_string(),
                                    script,
                                    scope,
                                    &mut state,
                                    )))
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
                        }
                        _ => pass()
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
