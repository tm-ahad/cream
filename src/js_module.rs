use crate::import_lib::import_lib;
use crate::state::_state;
use crate::state_base::_StateBase;
use std::fs::read_to_string;
use rusty_v8 as v8;

pub fn module(
    mut app: String,
    js: String
) -> (String, String) {

    let js_ = js;

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let mut base = _StateBase::new();

    while app.contains("import mod:") {
        match app.find("import mod:") {
            None => {}
            Some(e) => {
                let mut ci = e + 9;

                while &app[ci..ci + 1] != "\n" {
                    ci += 1
                }

                let name = &app.clone()[e + 11..ci];
                app.replace_range(e..ci + 1, "");

                let module = read_to_string(name)
                    .expect(&*format!("Module {name} not found"));

                let mut js = String::new();

                js = import_lib(module.clone(), module.clone(), true).0;
                js = _state(js, &mut base, scope)
            }
        }
    }

    (app, js_)
}