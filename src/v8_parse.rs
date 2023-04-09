use rusty_v8::{ContextScope, HandleScope, Script, String};

pub fn v8_parse(
    scope: &mut ContextScope<HandleScope>,
    s: &str
) -> std::string::String {

    let v8_str = String::new(scope, s)
        .unwrap();

    let script = Script::compile(scope, v8_str, None);
    let res = script
        .unwrap()
        .run(scope)
        .unwrap()
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    res
}