use crate::component_markup::ComponentMarkUp;
use crate::helpers::find_all::find_all;
use crate::helpers::html_atrribute_dom_prop_map::html_attribute_dom_prop_map;
use crate::helpers::interpolate_string::interpolate_string;
use crate::helpers::is_byte_in_str::{is_byte_in_str, UpdateIBIS};
use crate::replacement_flag::SingleReplacementMap;
use crate::state_base::_StateBase;
use crate::template_type::TemplateType;
use crate::var_not_allowed::var_not_allowed;
use crate::consts::{NEW_LINE, NIL, SPACE};
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;


pub fn split_once(s: String, delimiter: char, sd: String) -> (String, String) {
    match s.find(delimiter) {
        Some(a) => (s[..a].to_string(), s[a + 1..].to_string()),
        None => (sd, s),
    }
}

pub fn template(
    html: &mut ComponentMarkUp,
    script: &mut String,
    base: &mut _StateBase,
    f_name: &str,
) {
    let dyn_html = &mut html.dynamic;
    let html = &mut html.stat;
    let html_len = html.len();

    let ao = find_all(html, "$", f_name);
    let mut repmap = Vec::new();

    'outer: for a in ao {
        if &html[a - 1..a] == NEW_LINE {
            let mut ti = a;
            let id_f_d = read_until(&html, a+1, SPACE, f_name, ComponentPart::Unknown);

            while &html[ti..ti + 1] != ":" {
                if ti == a + 5 {
                    break 'outer;
                }

                ti += 1;
            }

            let id_x = read_until(&html, id_f_d, SPACE, f_name, ComponentPart::Unknown);

            let mut n = id_x;
            let mut upd = UpdateIBIS::new(is_byte_in_str(n, html));

            while &html[n..n + 1] != ";" || upd.update(&html[n..n + 1]) {
                if n == html_len-2 {
                    StdErr::exec(SyntaxError, &format!("; expected in template ({f_name})"))
                }
                n += 1;
            }

            let mut v = html[id_x..n].to_string();

            //For static html

            let temp_type = TemplateType::from_str(&html[a + 1..ti]);
            let attr_prop_map = html_attribute_dom_prop_map();
            let is_dyn = temp_type.is_dynamic();

            let mut prop;

            (prop, v) = split_once(v, '=', String::from("innerText"));

            //For dynamic html
            dyn_html.replace_range(a..n + 1, &interpolate_string(&v
                .replace('$', NIL)
            ));
            //done for dynamic html

            prop = match attr_prop_map.get(&*prop) {
                Some(p) => p.to_string(),
                None => prop,
            };

            while let Some(i) = v.find('$') {
                let mut idx = i;
                let char_array: [char; 64] = var_not_allowed();
                let vlen = v.len();

                let bytes = v.as_bytes();

                while idx < vlen && char_array.contains(&(bytes[idx] as char)) {
                    idx += 1;
                }

                let vn = &v[i + 1..idx];

                if vn.chars().next().unwrap().is_ascii_digit() {
                    panic!("Invalid variable name: {}", vn)
                }

                let id = if is_dyn {
                    &html[a + 5..id_f_d]
                } else {
                    &html[a + 2..id_f_d]
                };

                let c = v.chars().nth(1).unwrap();

                let main_v = if c == '$' {
                    v[2..].to_string()
                } else {
                    v[1..].to_string()
                };

                repmap.push(SingleReplacementMap::new(a..n + 1, String::new()));

                script.push_str(&format!(
                    "document.getElementById({id}).{prop}={};",
                    &main_v
                ));

                base._set(
                    vn.to_string(),
                    format!("document.getElementById({id}).{prop}"),
                    main_v.clone(),
                );

                v.remove(i);
            }
        } else {
            continue;
        }
    }

    for r in repmap {
        let (range, replace_with) = r.to_tuple();

        html.replace_range(range, &replace_with);
    }
}
