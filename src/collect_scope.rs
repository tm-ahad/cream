use crate::brace_pool::BracePool;
use crate::is_byte_in_str::is_byte_in_str;
use crate::matcher::Matcher;
use crate::mp::Mp;

pub fn collect_scope(toks: &str, matcher: &Matcher) -> Option<Mp> {
    let matchr = matcher.as_str();

    return match toks.find(matchr) {
        Some(s) => match matcher {
            Matcher::Component(_) => {
                let len = matchr.len();
                let remain = &toks[s + len..];

                return match remain.find('{') {
                    Some(ss) => {
                        let dif = &remain[..ss];

                        if dif.trim().is_empty() {
                            let mut pool = BracePool::new();
                            let mut cidx = 0;

                            for c in remain.chars() {
                                if c == '{' {
                                    pool.push(c);
                                } else if c == '}' {
                                    if pool.push('}') && !is_byte_in_str(cidx, remain) {
                                        return Some(Mp::new(remain[ss + 1..cidx].to_string(), s + cidx));
                                    }
                                }
                                cidx += 1;
                            }

                            None
                        } else {
                            collect_scope(&toks[s + ss..], &Matcher::Component(&matchr.to_string()))
                        }
                    }
                    None => collect_scope(remain, &Matcher::Component(&matchr.to_string())),
                };
            }
            Matcher::Template => {
                if !is_byte_in_str(s, toks) {
                    let remain = &toks[s..];

                    match remain.find("</temp>") {
                        Some(a) => {
                            if !is_byte_in_str(a, toks) {
                                return Some(Mp::new(remain[6..a].to_string(), s + 6));
                            } else {
                                return collect_scope(remain, &Matcher::Template);
                            }
                        }
                        None => panic!("</temp> expected to end the template scope"),
                    }
                }

                collect_scope(&toks[s..], &Matcher::Template)
            }
            Matcher::Dom => collect_scope(toks, &Matcher::Component(&"dom".to_string())),
            Matcher::Sin => collect_scope(toks, &Matcher::Component(&"sin".to_string())),
            Matcher::Cam => collect_scope(toks, &Matcher::Component(&"cam".to_string())),
        },
        None => None,
    };
}
