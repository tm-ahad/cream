use crate::state_base::_StateBase;
use crate::v8_parse::v8_parse;
use crate::consts::IGNORE_STATE;
use crate::var_not_allowed::var_not_allowed;
use rusty_v8::{ContextScope, HandleScope};

pub fn at_temp(
    html: &mut String,
    js: &mut String,
    base: &mut _StateBase,
    scope: &mut ContextScope<HandleScope>
) {
    while let Some(a) = html.find("@temp:") {
        let mut id_f_d = a + 6;

        while &html[id_f_d..id_f_d+1] != " " {
            id_f_d += 1;
        }

        let mut id_x = id_f_d;

        while &html[id_x..id_x+1] == " " {
            id_x += 1;
        }

        let mut n = id_x;

        while &html[n..n+1] != "\n" {
            n += 1;
        }

        let mut v = html[id_x..n].to_string();

        if let Some(i) = v.find('$') {
            let mut idx = i;
            let char_array: [char; 64] = var_not_allowed();

            while idx+1 < v.len() &&
                char_array.contains(&v
                    .chars()
                    .nth(idx+1)
                    .unwrap()
                )
            {
                idx += 1;
            }

            let mut vn = &v[i+1..idx+1];
            let mut is_dyn = false;

            if vn
                .chars()
                .next()
                .unwrap()
                .is_ascii_digit()
            {
                panic!("Invalid variable name: {}", vn)
            }

            if vn.starts_with('$') {
                vn = &vn[1..];
                is_dyn = true;
            }

            let id = &html[a+6..id_f_d];

            let c = v.chars()
                .nth(1)
                .unwrap();

            let main_v = if c == '$' {
                v[2..].to_string()
            } else { v[1..].to_string() };

            base._set(
                vn.to_string(),
                format!("document.getElementById({id}).innerHTML"),
                main_v.clone(),
            );

            if is_dyn {
                js.push_str(&format!("document.getElementById({id}).innerHTML={}{IGNORE_STATE}", &main_v));
                html.replace_range(a..n+1,"");
            } else {
                html.replace_range(a..n+1, &v8_parse(scope, vn));
            }

            v.remove(i);
        }
    }
}
