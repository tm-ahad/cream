use rusty_v8::{ContextScope, HandleScope, Script, String};

fn v8_parse_optional(
    scope: &mut ContextScope<HandleScope>,
    s: &str,
) -> Option<std::string::String> {
    let v8_str = String::new(scope, s).unwrap();

    let script = Script::compile(scope, v8_str, None);
    let val = script.unwrap().run(scope);

    val.map(|v| v.to_string(scope).unwrap().to_rust_string_lossy(scope))
}

pub fn v8_parse(scope: &mut ContextScope<HandleScope>, s: &str) -> std::string::String {
    v8_parse_optional(scope, s).unwrap()
}
