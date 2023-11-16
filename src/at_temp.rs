use crate::escape_string::escape_string_mut;
use crate::helpers::find_all_by_char::find_all;
use crate::helpers::is_byte_in_str::{is_byte_in_str, UpdateIBIS};
use crate::var_not_allowed::var_not_allowed;
use crate::component_markup::ComponentMarkUp;
use crate::helpers::interpolate_string::interpolate_string;
use crate::v8_parse::v8_parse;
use crate::consts::{NIL, SPACE};
use rusty_v8::{ContextScope, HandleScope};

pub fn at_temp(
    cmu: &mut ComponentMarkUp,
    script: &mut String,
    scope: &mut ContextScope<HandleScope>
) {
    let html = cmu.stat.clone();
    let ao = find_all(&html, "@temp:");

    for a in ao {
        let mut id_f_d = a + 6;

        while &html[id_f_d..id_f_d + 1] != SPACE {
            id_f_d += 1;
        }

        let mut id_x = id_f_d;

        while &html[id_x..id_x + 1] == SPACE {
            id_x += 1;
        }

        let mut n = id_x;
        let mut upd = UpdateIBIS::new(is_byte_in_str(n, &html));


        while !(&html[n..n + 1] == ";" && !upd.update(&html[n..n + 1])) {
            n += 1;
        }

        let mut v = html[id_x..n].to_string();
        let mut rep = false;

        let ao = find_all(&v, "$");

        for i in ao {
            let mut idx = i;
            let char_array: [char; 64] = var_not_allowed();

            let bytes = v.as_bytes();
            let vlen = v.len();

            while idx < vlen && char_array.contains(&(bytes[idx] as char)) {
                idx += 1;
            }

            let mut vn = &v[i..idx];
            let mut is_dyn = false;

            if vn.starts_with('$') {
                vn = &vn[1..];
                is_dyn = true;
            }

            if vn.is_empty() {
                continue;
            }

            if vn.chars().next().unwrap().is_ascii_digit() {
                panic!("Invalid variable name: {}", vn)
            }

            let id = &html[a + 6..id_f_d];
            let mut main_v = v[1..].to_string();

            escape_string_mut(&mut main_v);

            // base._set(
            //     vn.to_string(),
            //     format!("document.getElementById({id}).innerHTML"),
            //     main_v.clone(),
            // );

            if is_dyn {
                script.push_str(&format!(
                    "\ndocument.getElementById({id}).innerHTML={};",
                    &main_v
                ));

                if !rep {
                    cmu.dynamic.replace_range(a..n+1, &interpolate_string(&main_v));
                    cmu.stat.replace_range(a..n+1, NIL);
                    rep = true;
                }
            } else if !rep && !is_dyn {
                cmu.stat.replace_range(a..n+1, &v8_parse(scope, vn));
                cmu.dynamic.replace_range(a..n+1, &v8_parse(scope, vn));
                rep = true;
            }

            v.remove(i);
        }
    }
}
