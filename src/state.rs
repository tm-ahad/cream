use crate::var_not_allowed::var_not_allowed;
use crate::state_base::_StateBase;

pub fn _state(js: &mut String, b: &mut _StateBase) {
    let spl = js.lines();
    let mut lines = vec![];

    for li in spl {
        match li.find('=') {
            Some(a) => {
                if  &li[a..a + 2]   !=    "==" &&
                    &li[a..a + 2]   !=    "=>" &&
                    &li[a..a + 2]   !=    "<=" &&
                    &li[a-1..a + 1] !=    ">=" &&
                    &li[a-1..a + 1] !=    "!=" &&
                     !(li.starts_with("const ")
                        || li.starts_with("let ")
                        || li.starts_with("var "))
                    && !li.ends_with(":sin")
                    && li.contains('$')
                {
                    let len = li.len();
                    let c = String::from(li[a + 1..len].trim());

                    let mut dl = false;

                    while let Some(a) = c.find('$') {
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

                        let vn = &c[a+1..idx+1];

                        if vn
                            .chars()
                            .next()
                            .unwrap()
                            .is_ascii_digit()
                        {
                            panic!("Invalid variable name: {}", vn)
                        }

                        b._set(vn.to_string(), li[..a].to_string(), c.clone());

                        let p = b.parse(&ls, String::new(), c.clone());

                        lines.push(p);
                    }

                    if !dl {
                        lines.push(li.to_string())
                    }
                } else if li.ends_with(":sin") {
                    let l = li.len();
                    lines.push(li[..l-4].to_string());

                    continue
                }

                lines.push(li.to_string());
            }
            None => lines.push(li.to_string()),
        }
    }

    js.clear();
    js.push_str(&lines.join("\n"));
}
