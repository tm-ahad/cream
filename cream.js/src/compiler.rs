use crate::at_html::at_html;
use crate::collect_gen::collect_gen;
use crate::component::Component;
use crate::component::{component, parse};
use crate::get_prop::get_prop;
use crate::import_lib::import_lib;
use crate::import_script::import_script;
use crate::js_module::module;
use crate::IdGen;
use crate::sys_exec::sys_exec;
use crate::import_npm::import_npm;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::import_base::ImportBase;
use crate::std_err::{StdErr, ErrType::OSError};
use crate::template::template;
use crate::at_gen_id::_gen_id;
use rusty_v8 as v8;
use rusty_v8::json::stringify;
use rusty_v8::Script;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::{read_to_string, write};

pub fn compile(mut state: _StateBase, mut import_base: ImportBase, map: HashMap<String, String>) {
    let ext = get_prop(map.clone(), "lang");

    let binding = String::new();
    let command = map.get("build")
        .unwrap_or(&binding);

    let mut app = read_to_string(format!("./src/app.{ext}")).expect("Project or app.nts not found");

    let mut imports: Vec<Component> = vec![];
    let mut names: Vec<String> = vec![];

    let main_app = collect_gen(app.clone(), "app{".to_string(), "}", Some(0), false);
    let split = main_app.split('\n');

    let mut js = String::new();

    for s in split {
        if s != "<temp>" {
            js.push('\n');
            js.push_str(s)
        } else {
            break;
        }
    }

    let mut comp_html = format!(
        "{}\n",
        collect_gen(main_app.clone(), "<temp>".to_string(), "<temp/>", None, true)
    );

    import_lib(&mut app, &mut import_base, &mut js);

    module(&mut app, &mut import_base, &mut js);
    import_script(&mut app, &mut import_base, &mut js);
    _gen_id(&mut js, &mut comp_html);

    write(format!("./build/.$.{ext}"), js.clone())
        .unwrap_or_else(|e| panic!("{}", e));

    sys_exec(format!("{command} ./build/.$.{ext}"));

    js = read_to_string("./build/.$.js")
        .unwrap_or(js.clone());

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let ben = &js.replace(".cam()", "");
    let code = v8::String::new(scope, ben).unwrap();

    let mut script = Script::compile(scope, code, None).unwrap();

    let _ = script.run(scope).unwrap();

    while let Some(e) = app.find("import component") {
        let mut namei = e + 17;

        while &app[namei..namei + 4] != "from" {
            namei += 1;
        }

        let mut ci = namei+5;

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        let cns = app[e+16..namei].split(',');
        let fnm = &app[namei+5..ci];

        for cn in cns {
            names.push(cn.trim().to_string());
            imports.push(component(
                fnm.to_string(),
                cn.trim().to_string(),
                scope,
                &mut state,
                &mut import_base,
                command,
                &ext
            ));
        }

        app.replace_range(e..ci + 1, "")
    }

    at_html(&mut comp_html, &mut js, scope, &mut state);
    template(&mut comp_html, &mut js, scope, &mut state);

    _state(&mut js, &mut state, scope);

    js = js.replace(".sin()", "")
           .replace(".cam()", "");


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
                                    s.to_string(),
                                    "Render".to_string(),
                                    scope,
                                    &mut state,
                                    &mut import_base,
                                    command,
                                    &ext
                                ))),
                            );
                        }

                        js = format!(
                            "{js}\nvar Route = {}",
                            serde_json::to_string::<Value>(&Value::Object(map)).unwrap()
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

                let not_found = match map.get("404") {
                    Some(e) => parse(&component(
                        e.clone(),
                        String::from("Page"),
                        scope,
                        &mut state,
                        &mut import_base,
                        command,
                        &ext
                    )),
                    None => "\
                        <pre style=\"word-wrap: break-word; white-space: pre-wrap;\">404 page not found</pre>
                    ".to_string()
                };

                let v8_str = v8::String::new(scope, name_).unwrap();

                script = Script::compile(scope, v8_str, None).unwrap();

                let res = script.run(scope).unwrap();

                let router = stringify(scope, res).unwrap().to_rust_string_lossy(scope);

                let binding: Value = serde_json::from_str::<Value>(&router).unwrap();

                let obj = binding.as_object().unwrap();

                let mut map = Map::new();
                map.insert(String::from("404"), Value::String(not_found));

                for (key, val) in obj {
                    let s = val.as_str().unwrap();
                    let _ = map.insert(
                        key.clone(),
                        Value::String(parse(&component(
                            String::from(s),
                            String::from("Render"),
                            scope,
                            &mut state,
                            &mut import_base,
                            command,
                            &ext
                        ))),
                    );
                }

                js = format!(
                    "{}\nvar Route = {}",
                    js,
                    serde_json::to_string::<Value>(&Value::Object(map)).unwrap()
                );

                js = format!(
                    "{js}\n{}",
                    "\
    function main() {
        var path = window.location.pathname
        let not_found = true
        for (let k in Route) {
            if (path == k) {
                not_found = false
                document.body.innerHTML = Route[k]
                window.history.pushState({}, \"\", k)
            }
        }
        if (not_found) {
            document.body.innerHTML = Route[\"404\"]
            window.history.pushState({}, \"\", path)
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
        let m = &format!("<{}/>", n);
        let rep = comp_html.replace(" ", "");

        if let Some(e) = rep.find(m) {
            for i in &imports {
                if i.name == n {
                    comp_html.replace_range(e..e+m.len()+1, &i.html);
                    js = format!("{js}\n{}", i.js);
                }
            }
        }
    }

    let first = true;

    while let Some(e) = comp_html.find("<Until ") {
        let mut fall = e;

        while &comp_html[fall..fall+1] != "\n" && fall > 0 {
            fall -= 1;
        }

        let mut up = e + 7;
        let len = comp_html.len();

        while &comp_html[up..up+1] != ">" && up < len {
            up += 1;
        }

        let li = &comp_html[fall..up+1];
        let mut th = String::new();
        let mut do_ = String::new();

        match li.find("that=") {
            None => {}
            Some(e) => {
                let mut init = e + 5;

                while &li[init..init+1] != " " &&
                      &li[init..init+1] != "/" &&
                      &li[init..init+1] != "\n" {
                    init += 1
                }

                th = String::from(&li[e+5..init])
            }
        }

        match li.find("do=") {
            None => {}
            Some(e) => {
                let mut init = e + 3;

                while &li[init..init+1] != " " &&
                      &li[init..init+1] != "/" &&
                      &li[init..init+1] != "\n" {
                    init += 1
                }

                do_ = String::from(&li[e+3..init])
            }
        }

        let mut th_comp = &Component::NEW;
        let mut do_comp = &Component::NEW;

        for i in &imports {
            if i.name == th {
                th_comp = i
            }
        }

        for i in &imports {
            if i.name == do_ {
                do_comp = i
            }
        }

        let id = IdGen::get_and_update();

        let cb1 = "{";
        let cb2 = "}";

        comp_html.replace_range(fall..up, &format!("<div id={}>{}</div>", id , do_comp.html));

        if first {
            js.push_str("
class Work {

    #value;

    constructor(init) {
        this.#value = init;
    }

    do(then) {
        try {
            let _res = this.#value();

            let res = then({
                state: \"done\",
                error: null,
                value: _res
            });

            return res;
        } catch (e) {
           throw e;
        }
    }
}\n")
        }
        js.push_str(&format!("\
let work = new Work(function() {cb1}
    {}
{cb2})

work.do(function() {cb1}
    let ptr = document.getElementById(\"{id}\")
    ptr.innerHTML = `{}`
{cb2})
        ", th_comp.js, th_comp.html));
    }

    let head = get_prop(map.clone(), "head");

    import_npm(&mut app, &mut js);

    let binding = String::from("./build/dist.html");
    let _app_html = map.get("_app_html")
        .unwrap_or(&binding);

    write(
        _app_html,
        format!(
            "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta name=\"description\" content=\"{}\">
    <meta name=\"keywords\" content=\"{}\">
    <meta name=\"author\" content=\"{}\">
    <title>{}</title>
    {head}
</head>
<body>
    {comp_html}
    <script>
        {js}
    </script>
<body>
</html>
",
            get_prop(map.clone(), "description"),
            get_prop(map.clone(), "keywords"),
            get_prop(map.clone(), "author"),
            get_prop(map.clone(), "title")
        ),
    )
        .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));
}
