use crate::state_base::_StateBase;
use crate::v8_parse::v8_parse;
use crate::IdGen;
use rusty_v8::{ContextScope, HandleScope};

pub fn template(
    html: &mut String,
    js: &mut String,
    scope: &mut ContextScope<HandleScope>,
    base: &mut _StateBase,
) {
    while let Some(a) = html.find('$') {
        let mut ch = (">", "<", "<");
        let le = html.clone();

        let mut is_not_inner_text = true;

        let mut i = a;

        while &html[i..i + 1] != "=" && &html[i..i + 1] != ">" {
            is_not_inner_text = &html[i..i + 1] != "=";

            i += 1;
        }

        let prop = if is_not_inner_text {
            ch = ("=", ">", " ");
            let mut s = a;

            while &html[s - 1..s] != " " {
                s -= 1;
            }

            &le[s..a]
        } else {
            "innerText"
        };

        let mut idx = a;

        let mut pig = a;
        let mut zig = a;

        while &html[idx..idx + 1] != ch.1 && &html[idx..idx + 1] != " " {
            idx += 1;
        }

        while &html[zig..zig + 1] != ch.1 && &html[zig..zig + 1] != ch.2 {
            zig += 1;
        }

        while &html[pig..pig + 1] != ch.0 {
            pig -= 1
        }

        let mut len: usize = 0;
        let cloned = html.clone();

        let val = &cloned[a + 1..idx];
        let start = &cloned[pig + 1..a];

        let end = &cloned[idx..zig];

        let mut fall = a;
        let mut up = a;

        let html_len = html.len() - 1;

        while &html[fall..fall + 1] != "\n" && fall > 0 {
            fall -= 1
        }

        while &html[up..up + 1] != "\n" && up < html_len {
            up += 1
        }

        let sh = &html[fall..up];

        let id = match sh.find("id=\"") {
            Some(au) => {
                let mut init = au + 4;

                while &html[init..init + 1] != "\"" {
                    init += 1
                }

                String::from(&html[au + 4..init])
            }
            None => {
                let r = IdGen::get_and_update();

                len = r.len() + 6;
                html.insert_str(
                    match prop {
                        "innerText" => pig,
                        _ => zig,
                    },
                    &format!(" id=\"{}\"", r),
                );
                r
            }
        };

        let mut s = String::from("`");

        if prop == "innerText" {
            pub fn push_s(s: String, ps: &str, b: bool) -> String {
                let mut ls = s;
                let d = if b {
                    format!("\"{ps}\"")
                } else {
                    String::from(ps)
                };

                ls.push_str("${");
                ls.push_str(&d);

                ls.push('}');

                ls
            }

            if !start.is_empty() {
                s = push_s(s, start, true);
            }
            s = push_s(s, val, false);

            if !end.is_empty() {
                s = push_s(s, end, true);
            }
            s.push('`')
        }

        let fin = &if prop == "innerText" {
            s.replace(".dyn()", "")
        } else {
            val.replace(".dyn()", "")
        };

        let mut result = if !val.ends_with(".dyn()") {
            v8_parse(scope, fin)
        } else {
            String::new()
        };

        let p_val = val.replace(".dyn()", "");

        base._set(
            p_val.clone(),
            format!("document.getElementById({:?}).{prop}", id),
            if prop == "innerText" {
                s.replace(".dyn()", "")
            } else {
                p_val.clone()
            },
        );

        result = if (!end.is_empty() || !start.is_empty()) && prop != "innerText" {
            let wed = format!("\"{}\"", result);
            wed
        } else {
            result
        };

        if !val.ends_with(".dyn()") {
            html.replace_range(
                match prop {
                    "innerText" => pig + len + 1..zig + len,
                    _ => pig + 1..zig,
                },
                &result,
            );
        } else {
            html.replace_range(
                match prop {
                    "innerText" => pig + len + 1..zig + len,
                    _ => pig - prop.len()..zig,
                },
                "",
            );

            js.push_str(&format!(
                "\ndocument.getElementById({:?}).{prop}{}.sin()",
                id, fin
            ));
        }
    }
}
