use crate::scope::Pair;
use crate::IdGen;
use crate::v8_parse::v8_parse;
use crate::state_base::_StateBase;
use rusty_v8::{ContextScope, HandleScope};

pub fn template(
    mut html: String,
    mut js: String,
    scope: &mut ContextScope<HandleScope>,
    base: &mut _StateBase,
) -> Pair {

    while html.contains("$") {
        match html.find("$") {
            Some(a) => {
                let mut ch = (">", "<", "<");
                let le = html.clone();

                let prop = if html[..a].contains("=") {
                    ch = ("=", ">", " ");
                    let mut s = a;

                    while &html[s-1..s] != " " {
                        s -= 1;
                    }

                    &le[s..a]
                } else {"innerText"};

                let mut idx = a;

                let mut pig = a;
                let mut zig = a;

                while &html[idx..idx + 1] != ch.1 && &html[idx..idx + 1] != " " {
                    idx += 1;
                }

                while &html[zig..zig + 1] != ch.1 && &html[zig..zig + 1] != ch.2
                {
                    zig += 1;
                }

                while &html[pig..pig + 1] != ch.0 {
                    pig -= 1
                }

                let mut len: usize = 0;
                let cloned = html.clone();

                let val = &cloned[a + 1..idx];
                let start = &cloned[pig + 1..a];

                let end = if prop == "innerText" {
                    &cloned[idx..zig]
                } else {
                    &cloned[idx..zig]
                };

                let mut fall = a;
                let mut up = a;

                while &html[fall..fall + 1] != "\n" {
                    fall -= 1
                }

                while &html[up..up + 1] != "\n" {
                    up += 1
                }


                let sh = &html[fall..up];

                let id = match sh.find("id=\"") {
                    Some(au) => {
                        let mut init = au + 4;

                        while &html[init..init + 1] != "\"" {
                            init += 1
                        }

                        html[au + 4..init].to_string()
                    }
                    None => {
                        let r = IdGen::get_and_update();

                        len = r.len() + 6;
                        html.insert_str(match prop {
                            "innerText" => pig,
                            _ => zig
                        }, &*format!(" id=\"{}\"", r));
                        r
                    }
                };

                let mut s = String::from("`");

                if prop == "innerText" {
                    pub fn push_s(s: String, ps: &str, b: bool) -> String {
                        let mut ls = s;
                        let d = if b {
                            format!("\"{ps}\"")
                        } else {ps.to_string()};

                        ls.push_str("${");
                        ls.push_str(&*d);

                        ls.push_str("}");

                        ls
                    }

                    if !start.is_empty() {
                        s=push_s(s, start, true);
                    }
                    s=push_s(s, val, false);

                    if !end.is_empty() {
                        s=push_s(s, end, true);
                    }
                    s.push_str("`");
                }

                let fin = &*if prop == "innerText" {s.replace(".dyn()", "")} else {
                    val.replace(".dyn()", "")
                };

                let mut result = v8_parse(scope, fin);

                let p_val = val.replace(".dyn()", "");

                base._set(
                    p_val.clone(),
                    format!("document.getElementById({:?}).{prop}", id),
                    if prop == "innerText" {s.replace(".dyn()", "")}
                        else {p_val.clone()}
                );

                result = if (end != "" || start != "")
                    && prop != "innerText" {

                    let wed=format!("\"{}\"", result);
                    wed
                } else {result};

                if !val.ends_with(".dyn()") {

                    html.replace_range( match prop {
                        "innerText" => pig+len+1..zig+len,
                        _ => pig+1..zig
                    }, &*result);
                } else {

                    html.replace_range( match prop {
                        "innerText" => pig+len+1..zig+len,
                        _ => pig-prop.len()..zig
                    }, "");

                    js.push_str(&*
                        format!("\ndocument.getElementById({:?}).{prop}{}.sin()", id, fin));
                }
            }
            None => return Pair(html, js),
        };
    }

    Pair(html, js)
}
