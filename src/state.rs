use crate::consts::IGNORE_STATE;
use crate::helpers::is_byte_in_str::is_byte_in_str;
use crate::state_base::_StateBase;
use crate::var_not_allowed::var_not_allowed;
use std::collections::BTreeMap;

pub fn _state(script: &mut String, b: &mut _StateBase) {
    let spl = script.split(";");
    let mut lines = vec![];
    let mut i = 0;

    let mut am: BTreeMap<usize, (String, String)> = BTreeMap::new();

    for li in spl {
        let mut li = li.to_string();

        if li.contains('\n') {
            continue
        };

        match li.find('=') {
            Some(e) => {
                let z = &li[e..e + 2] != "=="
                    && &li[e..e + 2] != ">="
                    && &li[e..e + 2] != "<="
                    && &li[e - 1..e + 1] != ">="
                    && &li[e - 1..e + 1] != "!="
                    && !(li.starts_with("const ")
                        || li.starts_with("let ")
                        || li.starts_with("var "))
                    && !li.ends_with(IGNORE_STATE)
                    && !is_byte_in_str(e, &li);

                let (dol, found_dol) = match li.find('$') {
                    Some(i) => (i, true),
                    None => (0, false),
                };

                if z && found_dol && !is_byte_in_str(dol, &li) {
                    let len = li.len();
                    let mut c = String::from(li[e + 1..len].trim());

                    let mut dl = false;

                    while let Some(a) = c.find('$') {
                        li.remove(e + a + 1);
                        c.remove(a);
                        dl = true;
                        let char_array = var_not_allowed();
                        let mut idx = a;
                        let ls = li[..e].trim().to_string();

                        while idx + 1 < c.len()
                            && char_array.contains(&c.chars().nth(idx + 1).unwrap())
                        {
                            idx += 1;
                        }

                        let vn = &c[a..idx + 1];

                        if vn.chars().next().unwrap().is_ascii_digit() {
                            panic!("Invalid variable name: {}", vn)
                        }

                        b._set(vn.to_string(), li[..e].trim().to_string(), c.clone());

                        let p = b.parse(&ls, String::new(), &c);

                        lines.push(p);
                        c.remove(a);
                    }

                    if !dl {
                        lines.push(li.to_string())
                    }
                } else if li.ends_with(IGNORE_STATE) {
                    let l = li.len();
                    lines.push(li[..l - 4].to_string());

                    continue;
                }

                if z {
                    let rs = String::from(li[e + 1..li.len()].trim());
                    let ls = String::from(li[..e].trim());
                    lines.insert(i, li.to_string());
                    am.insert(i, (ls, rs));
                } else {
                    lines.push(li.to_string());
                }
            }
            None => lines.push(li.to_string()),
        }

        i += 1;
    }

    for (i, p) in am {
        let ls = p.0;
        let rs = p.1;

        let parsed = b.parse(&ls, rs, "");
        lines.insert(i, parsed);
    }

    *script = lines.join("\n")
}
