use crate::at_gen_id::_gen_id;
use crate::at_temp::at_temp;
use crate::collect_scope::collect_scope;
use crate::config::Config;
use crate::expected::expect_some;
use crate::import_base::ImportBase;
use crate::import_lib::import_lib;
use crate::import_npm::import_npm;
use crate::import_script::import_script;
use crate::comment::comment;
use crate::js_module::module;
use crate::matcher::Matcher;
use crate::scope::{parse_scope, scopify};
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::sys_exec::sys_exec;
use crate::template::template;
use crate::consts::IGNORE_STATE;
use crate::udt::UDT;
use rusty_v8::{self as v8, ContextScope, HandleScope, Script};
use std::fs::{read_to_string, write};

pub struct Component {
    pub js: String,
    pub html: String,
    pub name: String,
}

impl Component {
    pub(crate) const NEW: Self = Self {
        js: String::new(),
        html: String::new(),
        name: String::new(),
    };
}

pub fn component(
    f_name: String,
    c_name: String,
    scope: &mut ContextScope<HandleScope>,
    st: &mut _StateBase,
    import_base: &mut ImportBase,
    command: &String,
    config: &Config,
) -> Component {
    let __js__ = &String::from("js");

    let ext = config.get("lang").unwrap_or(__js__);

    let path = format!("./{f_name}").replace('\"', "");

    let mut app = read_to_string(path).unwrap_or_else(|e| {
        StdErr::exec(OSError, &e.to_string());
        todo!()
    });

    app = app
        .lines()
        .map(|e| e.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    comment(&mut app);

    let mut _imports: Vec<Component> = vec![];
    let mut _names: Vec<String> = vec![];

    let macher = Matcher::Component(&c_name);

    let pat = expect_some(collect_scope(&app, &macher, false), &c_name);

    let app_started = pat.index();
    let main_app = pat.mp_val();

    let binding = &main_app;

    let split = binding.split('\n');

    let mut js = String::new();
    let binding = Matcher::Template.to_string();
    let t = binding.as_str();

    for s in split {
        if s != t {
            js.push('\n');
            js.push_str(s)
        } else {
            break;
        }
    }

    let mut html = expect_some(
        collect_scope(&main_app, &Matcher::Template, false),
        "Template",
    )
    .mp_val();

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

        let fnm = &app[namei + 5..ci];

        let cns = app[e + 17..ci].split(',');

        for cn in cns {
            _names.push(String::from(app[e + 16..namei].trim()));
            _imports.push(component(
                String::from(fnm),
                String::from(cn.trim()),
                scope,
                st,
                import_base,
                command,
                config,
            ))
        }
    }

    import_lib(&mut app, import_base, &mut js, app_started);
    module(&mut app, import_base, &mut js);
    import_script(&mut app, import_base, &mut js);

    let mut scopes: Vec<String> = Vec::new();

    parse_scope(&mut js, &mut scopes);
    _gen_id(&mut js, &mut html);

    write(format!("./build/.$.{ext}"), js.clone()).unwrap_or_else(|e| panic!("{}", e));

    sys_exec(format!("{command} ./build/.$.{ext}"));

    js = read_to_string("./build/.$.js").unwrap_or(js.clone());

    let string = v8::String::new(scope, &js).unwrap();

    let script = Script::compile(scope, string, None).unwrap();

    let _ = script.run(scope);

    at_temp(&mut html, &mut js, st, scope);
    template(&mut html, &mut js, scope, st);
    _state(&mut js, st);

    for n in _names {
        let m = &*format!("<{}/>", n);
        let rep = html.replace(' ', "");

        if rep.contains(m) {
            for i in &_imports {
                if i.name == n {
                    if let Some(e) = html.find(m) {
                        let mut cde = e+n.len()+1;

                        while &html[cde..cde+1] != ">" {
                            cde += 1;
                        }

                        html.replace_range(e..cde+1, &i.html);

                        js.push('\n');
                        js.push_str(i.js.as_str())
                    }
                }
            }
        }
    }

    js = js.replace(IGNORE_STATE, "").
            replace(".cam()", "");

    UDT(&mut html, &mut js, &_imports);
    import_npm(&mut app, &mut js);
    scopify(&mut js, scopes, config, st);

    Component {
        js,
        html: html.to_string(),
        name: c_name,
    }
}

pub fn stringify_component(s: &Component) -> String {
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
