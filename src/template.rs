use rusty_v8::{ContextScope, HandleScope, Script, self as v8};
use crate::scope::Pair;
use crate::state_base::_StateBase;

pub fn template(mut html: String, js: String, scope:
    &mut ContextScope<HandleScope>, base: &mut _StateBase) -> Pair {

    let mut test_js = js.clone();

    while html.contains("$") {
        match html.find("$") {
            Some(a) => {
                let mut idx = a;

                while &html[idx..idx + 1] != "<" {
                    idx += 1;
                }

                let mut fall = a;
                let mut up = a;

                let mut pig = a;

                while &html[fall..fall + 1] != "\n" {
                    fall -= 1
                }

                while &html[up..up + 1] != "\n" {
                    up += 1
                }

                while &html[pig..pig + 1] != ">" {
                    pig -= 1
                }


                match html[fall..up].find("id=\"") {
                    Some(au) => {
                        let mut init = au + 4;

                        while &html[init..init + 1] != "\"" {
                            init += 1
                        }

                        let val = &html[pig+2..idx];

                        let code = v8::String::new(scope, val)
                            .expect("Variable can't be founded");

                        let script = Script::compile(scope, code, None)
                            .unwrap();

                        let result = &script.run(scope)
                            .unwrap()
                            .to_string(scope)
                            .unwrap()
                            .to_rust_string_lossy(scope)
                            [..];

                        base._set(val.to_string(),
                            format!("document.getElementById({:?}).innerText", &html[au+4..init]));
                        //
                        base.parse(val.to_string(), String::from(".single()"));
                        println!("{}", base.parse);
                        test_js = format!("{test_js}\n{}", base.parse);

                        html.replace_range(pig+1..idx, result);

                        return Pair(html, test_js);
                    }
                    None => panic!("id not found on templating element"),
                }
            }
            None => return Pair(html, js),
        }
    }

    Pair(html, js)
}
