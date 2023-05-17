use crate::at_html::at_html;
use crate::collect_gen::collect_gen;
use crate::state::_state;
use crate::state_base::_StateBase;
use crate::template::template;
use crate::import_npm::import_npm;
use crate::IdGen;
use crate::import_base::ImportBase;
use crate::import_script::import_script;
use crate::sys_exec::sys_exec;
use crate::js_module::module;
use crate::at_gen_id::_gen_id;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::import_lib::import_lib;
use rusty_v8::{ContextScope, HandleScope, self as v8, Script};
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
    ext: &String,
) -> Component {

    let path = format!("./{f_name}").replace('\"', "");

    let mut app = read_to_string(path)
        .unwrap_or_else(|e| {
            StdErr::exec(OSError, &e.to_string());
            todo!()
        });

    let mut _imports: Vec<Component> = vec![];
    let mut _names: Vec<String> = vec![];

    let mut macher = c_name.clone();
    macher.push('{');

    let main_app = collect_gen(app.clone(), macher, "}", Some(0), false);
    let binding = main_app.clone();
    let split = binding.split('\n');

    let mut js = String::new();

    let mut html = collect_gen(main_app, String::from("<temp>"), "</temp>", None, false);

    for s in split {
        if s != "<temp>" {
            js.push('\n');
            js.push_str(s)
        } else {
            break
        }
    }

    import_lib(&mut app, import_base, &mut js);
    module(&mut app, import_base, &mut js);
    import_script(&mut app, import_base, &mut js);
    _gen_id(&mut js, &mut html);

    write(format!("./build/.$.{ext}"), js.clone())
        .unwrap_or_else(|e| panic!("{}", e));

    sys_exec(format!("{command} ./build/.$.{ext}"));

    js = read_to_string("./build/.$.js")
            .unwrap_or(js.clone());

    let string = v8::String::new(scope, &js)
        .unwrap();

    let script = Script::compile(scope, string, None)
        .unwrap();

    let _ = script
        .run(scope);

    template(&mut html, &mut js, scope, st);
    at_html(&mut html, &mut js, scope, st);
    _state(&mut js, st, scope);

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

        let cns = app[e+17..ci].split(',');

        for cn in cns {
            _names.push(String::from(app[e + 16..namei].trim()));
            _imports.push(component(
                String::from(fnm),
                String::from(cn.trim()),
                scope,
                st,
                import_base,
                command,
                &ext,
            ))
        }
    }

    for n in _names {
        let m = &*format!("<{}/>", n);

        if html.contains(m) {
            for i in &_imports {
                if i.name == n {

                     if let Some(e) = html.find(m) {
                         html.replace_range(e..m.len() + 1, &i.html);
                         js = format!("{js}\n{}", i.js)
                     }
                }
            }
        }
    }

    let first = true;

    while let Some(e) = html.find("<Until ") {
        let mut fall = e;

        while &html[fall..fall+1] == "\n" {
            fall -= 1;
        }

        let mut up = e + 7;

        while &html[up..up+1] == "\n" {
            up += 1;
        }

        let li = &html[fall..up];
        let mut th = String::new();
        let mut do_ = String::new();

        match li.find("that=") {
            None => {}
            Some(e) => {
                let mut init = e + 5;

                while &li[init..init+1] != " " && &li[init..init+1] != "/" {
                    init += 1
                }

                th = String::from(&li[e+5..init])
            }
        }

        match li.find("do=") {
            None => {}
            Some(e) => {
                let mut init = e + 3;

                while &li[init..init+1] != " " && &li[init..init+1] != "/" {
                    init += 1
                }

                do_ = String::from(&li[e+3..init])
            }
        }

        let mut th_comp = &Component::NEW;
        let mut do_comp = &Component::NEW;

        for i in &_imports {
            if i.name == th {
                th_comp = i
            }
        }

        for i in &_imports {
            if i.name == do_ {
                do_comp = i
            }
        }

        let id = IdGen::get_and_update();

        let cb1 = "{";
        let cb2 = "}";

        html.replace_range(fall..up, &format!("<div id={}>{}</div>",id , do_comp.html));

        if first {
            js.push_str("
class Work {

    #value;

    constructor(init, args = []) {
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
        }
        } catch (e) {
           throw e;
        }
    }
}")
        }
        js.push_str(&format!("\
let work = new Work(function() {cb1}
    {}
{cb2})

work.do(function() {cb1}
    let ptr = document.getElementById({id})

    ptr.innerHTML = {}
{cb2})
        ", th_comp.js, th_comp.html));
    }

    import_npm(&mut app, &mut js);

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
