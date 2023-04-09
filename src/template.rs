use crate::gen_id::gen_id;
use crate::scope::Pair;
use crate::state_base::_StateBase;
use rusty_v8::{ContextScope, HandleScope};
use crate::v8_parse::v8_parse;

pub fn template(
    mut html: String,
    js: String,
    scope: &mut ContextScope<HandleScope>,
    base: &mut _StateBase,
) -> Pair {
    let test_js = js.clone();

    while html.contains("$") {
        return match html.find("$") {
            Some(a) => {
                let mut idx = a;

                while &html[idx..idx + 1] != "<" {
                    idx += 1;
                }

                let mut zig = a;

                while &html[idx..idx + 1] != "<" && &html[idx..idx + 1] != " " {
                    idx += 1;
                }

                while &html[zig..zig + 1] != "<" {
                    zig += 1;
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

                let sh = &html[fall..up];
                let val = &html.clone()[a + 1..idx];
                let start = &html.clone()[pig + 1..a];
                let end = &html.clone()[idx + 1..zig];

                let mut len = 0 as usize;

                let id = match sh.find("id=\"") {
                    Some(au) => {
                        let mut init = au + 4;

                        while &html[init..init + 1] != "\"" {
                            init += 1
                        }

                        html[au + 4..init].to_string()
                    }
                    None => {
                        let r = gen_id();
                        
                        len = r.len() + 6;
                        html.insert_str(pig, &*format!(" id=\"{}\"", r));
                        r
                    }
                };
                
                let result = &*v8_parse(scope, &*format!("`${start}${val}${end}`"));
                base._set(
                    val.to_string(),
                    format!("document.getElementById({:?}).innerText", id),
                    val.to_string(),
                );

                html.replace_range(pig + 1 + len..idx + len, result);

                Pair(html, test_js)
            }
            None => Pair(html, js),
        };
    }

    Pair(html, js)
}
