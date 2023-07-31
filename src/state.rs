use crate::pass::pass;
use crate::state_base::_StateBase;
use crate::std_err::ErrType::SyntaxError;
use crate::std_err::StdErr;
use crate::var_not_allowed::var_not_allowed;
use rusty_v8::{ContextScope, HandleScope};
use serde_json::Value;

pub fn _state(js: &mut String, b: &mut _StateBase, scope: &mut ContextScope<HandleScope>) {
    let spl = js.split('\n').collect::<Vec<&str>>();
    let mut lines: Vec<String> = vec![];

    for li in spl.iter() {
        match li.find('=') {
            Some(a) => {
                if &li[a..a + 1] == "="
                    && !(li.starts_with("const ")
                        || li.starts_with("let ")
                        || li.starts_with("var "))
                    && !li.ends_with(".sin()")
                {
                    let len = li.len();
                    let c = String::from(li[a + 1..len].trim());

                    let a_ = var_not_allowed();

                    let mut idx = 0;
                    let mut refs: Vec<&str> = Vec::new();

                    match serde_json::from_str::<Value>(&c.clone()) {
                        Err(_) => {
                            for ch in c.chars() {
                                let mut is_in_str = false;

                                if ch == '"' || ch == '\'' {
                                    is_in_str = !is_in_str
                                }

                                if !a_.contains(&ch) {
                                    let mut end = idx;
                                    let mut ann = idx;
                                    let len__ = c.len() - 1;

                                    let mut chars = c.chars();

                                    while end != 0 && a_.contains(&chars.nth(end - 1).unwrap()) {
                                        end -= 1;
                                    }

                                    while ann != len__ && a_.contains(&chars.nth(ann + 1).unwrap())
                                    {
                                        ann += 1;
                                    }

                                    let test_ = &c[end..idx];
                                    let _test = &c[idx + 1..ann + 1];

                                    match serde_json::from_str::<Value>(test_) {
                                        Err(_) => refs.push(test_),
                                        _ => pass(),
                                    }

                                    match serde_json::from_str::<Value>(_test) {
                                        Err(_) => refs.push(_test),
                                        _ => pass(),
                                    }
                                }

                                idx += 1
                            }

                            let raw_val = &li[..a];

                            for k in refs {
                                b._set(
                                    String::from(k),
                                    String::from(raw_val),
                                    c.replace(".cam()", ""),
                                );
                            }
                        }
                        Ok(_) => pass(),
                    }

                    let rw = String::from(li[..a].trim());

                    if !li.ends_with(".cam()") {
                        b.parse(rw, String::new(), c.clone());
                    } else {
                        b.catch_parse(rw, String::new(), c.replace(".cam()", ""), scope);
                    }

                    lines.push(li.to_string());

                    if !b.parse.clone().is_empty() {
                        lines.push(b.parse.clone());
                    }
                } else if &li[a..a + 2] == ":=" && !li.ends_with(".sin()") {
                    let len = li.len();
                    let c = String::from(li[a + 1..len].trim());

                    let a_ = var_not_allowed();

                    let mut idx = 0;
                    let mut refs: Vec<&str> = Vec::new();

                    match serde_json::from_str::<Value>(&c.clone()) {
                        Err(_) => {
                            for ch in c.chars() {
                                let mut is_in_str = false;

                                if ch == '"' || ch == '\'' {
                                    is_in_str = !is_in_str
                                }

                                if !a_.contains(&ch) {
                                    let mut end = idx;
                                    let mut ann = idx;
                                    let len__ = c.len() - 1;

                                    let mut chars = c.chars();

                                    while end != 0 && a_.contains(&chars.nth(end - 1).unwrap()) {
                                        end -= 1;
                                    }

                                    while ann != len__ && a_.contains(&chars.nth(ann + 1).unwrap())
                                    {
                                        ann += 1;
                                    }

                                    let test_ = &c[end..idx];
                                    let _test = &c[idx + 1..ann + 1];

                                    match serde_json::from_str::<Value>(test_) {
                                        Err(_) => refs.push(test_),
                                        _ => pass(),
                                    }

                                    match serde_json::from_str::<Value>(_test) {
                                        Err(_) => refs.push(_test),
                                        _ => pass(),
                                    }
                                }

                                idx += 1
                            }

                            let raw_val = &li[..a];

                            for k in refs {
                                b._set(
                                    String::from(k),
                                    String::from(raw_val),
                                    c.replace(".cam()", ""),
                                );
                            }
                        }
                        Ok(_) => pass(),
                    }
                } else if &li[a..a + 2] == "::" && !li.ends_with(".sin()") {
                    if !li.ends_with(".cam()") {
                        b.parse(
                            String::from(&li[..a]),
                            String::new(),
                            String::from(&li[a + 2..]),
                        );
                    } else {
                        b.catch_parse(
                            String::from(&li[..a]),
                            String::new(),
                            li[a + 2..].replace(".cam()", ""),
                            scope,
                        );
                    }

                    lines.push(b.parse.clone())
                } else if li.starts_with("const")
                    || li.starts_with("let")
                    || li.starts_with("var")
                    || li.ends_with(".sin()")
                {
                    lines.push(li.to_string())
                } else {
                    StdErr::exec(SyntaxError, "Invalid Operator");
                }

                continue;
            }
            None => lines.push(li.to_string()),
        }
    }

    js.clear();
    js.push_str(&lines.join("\n"))
}
