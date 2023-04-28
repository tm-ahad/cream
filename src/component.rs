use crate::at_html::at_html;
use crate::collect_gen::collect_gen;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::template::template;
use crate::import_base::ImportBase;
use crate::import_script::import_script;
use crate::js_module::module;
use crate::import_lib::import_lib;
use rusty_v8::{ContextScope, HandleScope, self as v8, Script};
use std::fs::read_to_string;

pub struct Component {
    pub js: String,
    pub html: String,
    pub name: String,
}

pub fn component(
    f_name: String,
    c_name: String,
    scope: &mut ContextScope<HandleScope>,
    st: &mut _StateBase,
    import_base: &mut ImportBase
) -> Component {

    let path = format!("./{f_name}").replace('\"', "");

    let mut app = read_to_string(path).expect("file not found");
    let mut _imports: Vec<Component> = vec![];
    let mut _names: Vec<String> = vec![];

    let mut macher = c_name.clone();
    macher.push('{');

    let main_app = collect_gen(app.clone(), macher, "}", Some(0), false);
    let binding = main_app.clone();
    let split = binding.split('\n');

    let mut js = String::new();

    let libs = import_lib(app, import_base, js, false);

    app = libs.0;
    js = libs.1;

    let mut html = collect_gen(main_app, "<temp>".to_string(), "</temp>", None, false);

    for s in split {
        if s != "<temp>" {
            js.push('\n');
            js.push_str(s)
        } else {
            break
        }
    }

    let res = module(app.clone(), import_base, js);

    app = res.0;
    js = res.1;

    let scr = import_script(app.clone(), import_base, js);

    app = scr.0;
    js = scr.1;

    let string = v8::String::new(scope, &*js)
        .unwrap();

    let script = Script::compile(scope, string, None)
        .unwrap();

    let _ = script
        .run(scope);

    let caught = template(html, js.clone(), scope, st);

    js = caught.1;
    html = caught.0;

    let ht = at_html(html.clone(), js.clone(), scope, st);

    html = ht.0;
    js = ht.1;

    js = _state(js.clone(), st, scope);

    js = js.replace(".sin()", "")
        .replace(".cam()", "");

    while let Some(e) = app.find("import component") {
        let mut namei = e + 17;
        let mut ci = e + 30;

        while &app[namei..namei + 4] != "from" {
            namei += 1;
        }

        while &app[ci..ci + 1] != "\n" {
            ci += 1
        }

        app.replace_range(e..namei, "");

        let fnm = &app[namei+5..ci];

        let cns = app[e+17..ci].split(",");

        for cn in cns {
            _names.push(app[e + 16..namei].trim().to_string());
            _imports.push(component(
                fnm.to_string(),
                cn.trim().to_string(),
                scope,
                st,
                import_base
            ))
        }
    }

    let mut fail = String::new();

    for n in _names {
        fail = format!("<{}/>", n);
        let m = fail.as_str();

        if html.contains(<&str>::clone(&m)) {
            for i in &_imports {
                if i.name == n {

                     if let Some(e) = html.find(<&str>::clone(&m)) {
                         html.replace_range(e..m.len() + 1, &i.html);
                         js = format!("{js}\n{}", i.js)
                     }
                }
            }
        }
    }


    Component {
        js,
        html,
        name: c_name,
    }
}

pub fn parse(s: &Component) -> String {
    format!(
        "
{}
<script>
{}
<script>
    ",
        s.html, s.js
    )
}
