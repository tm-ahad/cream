use crate::consts::{IMP_STATE_SIGN, IMP_STATE_SIGN_LEN, NEW_LINE, NEW_LINE_CHAR, NIL};
use crate::helpers::add_line::add_line;
use crate::helpers::is_byte_in_str::is_byte_in_str;
use crate::pass::pass;
use crate::var_not_allowed::var_not_allowed;
use crate::state_base::_StateBase;

fn find_special_assignment(s: &str) -> Option<(usize, usize)> {
    let col_f = s.find(":");

    match col_f {
        Some(col_f) => {
            let col_e = s[col_f+1..].find("=");

            if let Some(col_e) = col_e {
                if s[col_f+1..col_f+1+col_e].trim().is_empty() {
                    return Some((col_f, col_f+1+col_e));
                }
            }

        }
        None => pass()
    }

    None
}


pub fn _state(scr: &mut String, b: &mut _StateBase) {
    let mut res = String::new();
    let mut end = String::new();

    while let Some(i) = scr.find(IMP_STATE_SIGN) {
        let line_start = i + IMP_STATE_SIGN_LEN;
        let mut line_end = line_start;
        let mut e = line_start;

        let script_len = scr.len();

        if line_start == script_len - 1 {
            scr.replace_range(i..line_start, NIL);
            continue
        }

        while !(
            &scr[line_end..line_end + 1] == ";" ||
            line_end == script_len - 1
        )
        {
            line_end += 1;
        }

        while !(
            &scr[e..e + 1] == "=" ||
            e == script_len - 1
        )
        {
            e += 1;
        }

        let mut c = String::from(scr[e + 1..line_end + 1].trim());
        let mut flin = scr[line_start..line_end].to_string();

        while let Some(a) = c.find('$') {
            c.remove(a);
            let char_array = var_not_allowed();
            let mut idx = a;
            let ls = scr[line_start..e].trim().to_string();

            while idx + 1 < c.len()
                && char_array.contains(&c.chars().nth(idx).unwrap())
            {
                idx += 1;
            }

            let vn = &c[a..idx];

            if vn.is_empty() {
                continue
            }
            flin.remove(a + e + 1 - line_start);

            if vn.chars().next().unwrap().is_ascii_digit() {
                panic!("Invalid variable name: {}", vn)
            }

            b._set(vn.to_string(), ls, c.clone());
        }

        add_line(&mut end, &flin);
        scr.replace_range(i..line_end+1, NIL);
    }

    let ao = scr.lines();
    let script_len = scr.len();

    let mut ci: usize = 0;

    for lin in ao {
        if let Some((mut e, mut f)) = find_special_assignment(lin) {
            e += ci;
            f += ci;

            if !is_byte_in_str(e, &scr) {
                let mut line_start = e;

                let mut line_end = f;

                while !(
                    line_end == script_len-1 ||
                    &scr[line_end..line_end+1] == ";"
                )
                {
                    line_end += 1;
                    if line_end == script_len-1 {
                        break
                    }
                }

                while !(
                    &scr[line_start-1..line_start] == NEW_LINE ||
                    line_start == 1
                )
                {
                    line_start -= 1;
                    if line_start == 1 {
                        break
                    }
                }

                let mut c = String::from(scr[f+1..line_end].trim());
                let ls = scr[line_start..e].trim().to_string();

                while let Some(a) = c.find('$') {
                    c.remove(a);
                    let char_array = var_not_allowed();
                    let mut idx = a;

                    while idx + 1 < c.len()
                        && char_array.contains(&c.chars().nth(idx + 1).unwrap())
                    {
                        idx += 1;
                    }

                    let vn = &c[a..idx + 1];

                    if vn.chars().next().unwrap().is_ascii_digit() {
                        panic!("Invalid variable name: {}", vn)
                    }

                    let p = b.parse(&ls, c.clone(), NIL);
                    b._set(vn.to_string(), ls.clone(), c.clone());

                    res.push_str(&p);
                    res.push(NEW_LINE_CHAR);
                    c.remove(a);
                }
            }
        } else {
            add_line(&mut res, &lin);
        }

        ci += lin.len()+1;
    }

    *scr = format!("{res}\n{end}");
}
