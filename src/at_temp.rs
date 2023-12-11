use crate::escape_string::escape_string_mut;
use crate::helpers::find_all::find_all;
use crate::helpers::is_byte_in_str::{is_byte_in_str, UpdateIBIS};
use crate::helpers::interpolate_string::interpolate_string;
use crate::var_not_allowed::var_not_allowed;
use crate::component_markup::ComponentMarkUp;
use crate::helpers::imp_sign::imp_sign;
use crate::consts::{NIL, SPACE};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;

pub fn at_temp(
    cmu: &mut ComponentMarkUp,
    script: &mut String,
    f_name: &str,
) {
    let html = cmu.stat.clone();
    let html_len = html.len();
    let ao = find_all(&html, "@temp:", f_name);

    for a in ao {
        let id_f_d = read_until(&html, a+6, SPACE, f_name, ComponentPart::Template);
        let id_x = read_until(&html, id_f_d, SPACE, f_name, ComponentPart::Template);

        let mut n = id_x;
        let mut upd = UpdateIBIS::new(is_byte_in_str(n, &html));

        while &html[n..n + 1] != ";" || upd.update(&html[n..n + 1]) {
            if n == html_len-2 {
                StdErr::exec(SyntaxError, &format!("; expected in template ({f_name})"))
            }
            n += 1;
        }

        let mut v = html[id_x+1..n].to_string();
        let ao = find_all(&v, "$", f_name);

        for i in ao {
            let mut idx = i;
            let char_array: [char; 64] = var_not_allowed();

            let bytes = v.as_bytes();
            let vlen = v.len();

            while idx < vlen && char_array.contains(&(bytes[idx] as char)) {
                idx += 1;
            }

            let vn = &v[i..idx];

            if vn.is_empty() {
                continue;
            }

            if vn.chars().next().unwrap().is_ascii_digit() {
                panic!("Invalid variable name: {}", vn)
            }

            let id = &html[a + 6..id_f_d];
            let mut main_v = v[1..].to_string();

            escape_string_mut(&mut main_v);

            script.push_str(&imp_sign(format!(
                "document.getElementById({id}).innerHTML={};",
                &main_v
            )));

            cmu.dynamic.replace_range(a..n+1, &interpolate_string(&main_v));
            cmu.stat.replace_range(a..n+1, NIL);

            v.remove(i);
        }
    }
}
