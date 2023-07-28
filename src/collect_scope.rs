use crate::brace_pool::BracePool;
use crate::consts::MAX_SCOPE_SIZE;
use crate::is_byte_in_str::is_byte_in_str;
use crate::matcher::Matcher;
use crate::mp::Mp;

pub fn collect_scope(toks: &str, matcher: &Matcher) -> Option<Mp> {
    let matchr = matcher.as_str();

    return match toks.find(matchr) {
        Some(s) => {
            match matcher {
                Matcher::Component(_) => {
                    let len = matchr.len();
                    let remain = &toks[s + len..];

                    return match remain.find('{') {
                        Some(ss) => {
                            let dif = &remain[..ss];

                            if dif.trim().is_empty() {
                                let mut pool = BracePool::new();
                                let mut i = ss + 1;

                                while &remain[i..i+1] == "{" {
                                    pool.push('{');
                                    i += 1;
                                }

                                loop {
                                    if &remain[i..i+1] == "}" {
                                        if i == MAX_SCOPE_SIZE {
                                            let a = "}";
                                            panic!("{a} expected to end the scope")
                                        } else if pool.push('}') && !is_byte_in_str(i, remain) {
                                            return Some(Mp::new(remain[ss + 1..i].to_string(), s + i));
                                        };
                                    }
                                    i += 1;
                                };

                                None
                            } else {
                                collect_scope(&toks[s + ss..], &Matcher::Component(&matchr.to_string()))
                            }
                        }
                        None => collect_scope(remain, &Matcher::Component(&matchr.to_string())),
                    };
                },
                Matcher::Template => {
                    if !is_byte_in_str(s, toks) {
                        let remain = &toks[s..];

                        match remain.find("</temp>") {
                            Some(a) => {
                                if !is_byte_in_str(a, toks) {
                                    return Some(Mp::new(remain[6..a].to_string(), s+6))
                                } else {
                                    return collect_scope(remain, &Matcher::Template)
                                }
                            }
                            None => {
                                panic!("</temp> expected to end the template scope");
                                todo!()
                            }
                        }
                    }

                    collect_scope(&toks[s..], &Matcher::Template)
                },
                _ => panic!("Invalid matcher")
            }
        }
        None => None,
    };
}
