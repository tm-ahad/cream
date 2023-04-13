use crate::gen_id::gen_id;
use crate::scope::Pair;
use crate::v8_parse::v8_parse;
use crate::state_base::_StateBase;
use rusty_v8::{ContextScope, HandleScope};

pub fn template(
    mut html: String,
    js: String,
    scope: &mut ContextScope<HandleScope>,
    base: &mut _StateBase,
) -> Pair {

    while html.contains("$") {
        return match html.find("$") {
            Some(a) => {
                let mut ch = (">", "<");
                let le = html.clone();

                let prop = if html[..a].contains("=") {
                    ch = ("=", ">");
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

                while &html[zig..zig + 1] != ch.1 {
                    zig += 1;
                }

                while &html[pig..pig + 1] != ch.0 {
                    pig -= 1
                }

                let mut len: usize = 0;
                let val = &html.clone()[a + 1..idx];
                let start = &html.clone()[pig + 1..a];
                let end = &html.clone()[idx..zig];

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
                        let r = gen_id();

                        len = r.len() + 6;
                        html.insert_str(match prop {
                            "innerText" => pig,
                            _ => zig
                        }, &*format!(" id=\"{}\"", r));
                        r
                    }
                };

                let mut s = String::from("`");

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

                s=push_s(s, start, true);
                s=push_s(s, val, false);
                s=push_s(s, end, true);
                s.push_str("`");

                let mut result = v8_parse(scope, &*s);

                base._set(
                    val.to_string(),
                    format!("document.getElementById({:?}).{prop}", id),
                    result.to_string(),
                );

                result = if (end != "" || start != "")
                    && prop != "innerText" {

                    let wed=format!("\"{}\"", result);
                    wed
                } else {result};

                html.replace_range( match prop {
                    "innerText" => pig+len+1..zig+len,
                    _ => pig+1..zig
                }, &*result);

                Pair(html, js)
            }
            None => Pair(html, js),
        };
    }

    Pair(html, js)
}
