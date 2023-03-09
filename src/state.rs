use crate::browser_::browser_objects;
use crate::collect_gen::concat_lines_exponent0;
use crate::state_base::_StateBase;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;

pub fn _state(js: String, b: &mut _StateBase) -> String {
    let spl = js.split("\n").collect::<Vec<&str>>();
    let mut lines: Vec<String> = vec![];

    for mut li in spl.iter() {
        #[allow(unused_assignments)]
        let mut v: &str = "";

        match li.find("=") {
            Some(a) => {
                let arth: [&str; 12] = ["+=", "-=", "*=", "^=", "|=", "&&=",
                    "||=", "^=", "~=", "<<=", ">>=", ">>>="];


                let name = &li[..a-1];

                if arth.contains(&&li[a-1..a+1]) && !li.ends_with(".single()") {
                    let mut bored: bool = true;

                    let mut l = li.to_string();

                    let backup: String = l.clone();
                    l.replace_range(a..a+1 , "");

                    v = l.as_str();
                    li = &v;

                    for l in b.map.clone() {

                        if l.0 == name.trim() {
                            bored = false;
                            lines.push(format!("update{}({})", name, li));
                        }
                    }

                    if bored {
                        lines.push(backup)
                    }
                }
                else if &li[a..a+1] == "=" && !(li.starts_with("const")
                    || li.starts_with("let")
                    || li.starts_with("var")) && !li.ends_with(".single()") {

                    let len = li.len();
                    let c = li[a+1..len].trim().to_string();
                    let ac = &li[0..a].trim();

                    b._set(c.clone(),li[..a].trim().to_string());
                    b.parse(c.clone());

                    lines.push(b.parse.clone());
                    let mut f = true;

                    for s in browser_objects() {
                        if li.contains(s) {
                            f = false;
                            lines.push(format!("update{}({})",c, c))
                        }
                    }

                    if f {
                        lines.push(format!("update{}({})",c, ac))
                    }
                }
                else if li.starts_with("const")
                    || li.starts_with("let")
                    || li.starts_with("var")
                    || li.ends_with(".single()") {

                    lines.push(li.to_string())
                }
                else {
                    let err = StdErr::new(SyntaxError, "Invalid Operator");

                    err.exec()
                }

                continue
            }
            None => {
                lines.push(li.parse().unwrap());
            }
        }

    }

    return concat_lines_exponent0(lines)
}
