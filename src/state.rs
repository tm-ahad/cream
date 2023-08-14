use crate::consts::IGNORE_STATE;
use crate::var_not_allowed::var_not_allowed;
use crate::state_base::_StateBase;

pub fn _state(js: &mut String, b: &mut _StateBase) {
    let spl = js.lines()
        .map(|l| l.to_string());
    let mut lines = vec![];

    for mut li in spl {
        li = li.trim()
            .to_string();

        match li.find('=') {
            Some(e) => {
                let z = &li[e..e + 2]   !=    "==" &&
                    &li[e..e + 2]   !=    "=>" &&
                    &li[e..e + 2]   !=    "<=" &&
                    &li[e-1..e + 1] !=    ">=" &&
                    &li[e-1..e + 1] !=    "!=" &&
                    !(li.starts_with("const ")
                        || li.starts_with("let ")
                        || li.starts_with("var "))
                    && !li.ends_with(IGNORE_STATE);

                let mut htpol = true;

                if z && li.contains('$')
                {
                    let len = li.len();
                    let mut c = String::from(li[e + 1..len].trim());

                    let mut dl = false;

                    while let Some(a) = c.find('$') {
                        li.remove(e+a+2);
                        c.remove(a);
                        dl = true;
                        let char_array = var_not_allowed();
                        let mut idx = a;
                        let ls = li[..a].to_string();

                        while idx+1 < c.len() &&
                            char_array.contains(&c
                                .chars()
                                .nth(idx+1)
                                .unwrap()
                            )
                        {
                            idx += 1;
                        }

                        let vn = &c[a..idx+1];

                        if vn
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            panic!("Invalid variable name: {}", vn)
                        }


                        b._set(
                            vn.to_string(),
                            li[..e]
                                .trim()
                                .to_string(),
                            c.clone()
                        );

                        let p = b.parse(&ls, String::new(), c.clone());

                        lines.push(p);
                        c.remove(a);
                    }

                    if !dl {
                        lines.push(li.to_string())
                    }
                } else if li.ends_with(IGNORE_STATE) {
                    let l = li.len();
                    lines.push(li[..l-4].to_string());

                    continue
                } else if z {
                    let ls = String::from(li[e + 1..li.len()].trim());
                    let rs = String::from(li[..e].trim());

                    let parsed = b.parse(&rs, String::new(), ls);

                    htpol = false;
                    lines.push(li.to_string());
                    lines.push(parsed);
                }

                if htpol {
                    lines.push(li.to_string());
                }
            }
            None => lines.push(li.to_string()),
        }
    }

    js.clear();
    js.push_str(&lines.join("\n"));
}
