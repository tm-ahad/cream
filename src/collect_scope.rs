use crate::helpers::is_byte_in_str::is_byte_in_str;
use crate::matcher::Matcher;
use crate::mp::Mp;

pub fn collect_scope(toks: &str, matcher: &Matcher, i_s: bool) -> Option<Mp> {
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
                            if let Some(i) = remain.rfind("}") {
                                return Some(Mp::new(
                                    remain[ss + 1..i - 1].to_string(),
                                    if i_s { s } else { s + ss + matchr.len() },
                                    if i_s { Some(s + len + ss + i) } else { None },
                                ));
                            }

                            None
                        } else {
                            collect_scope(
                                &toks[s + len..],
                                &Matcher::Component(matchr),
                                i_s,
                            )
                        }
                    }
                    None => collect_scope(remain, &Matcher::Component(matchr), i_s),
                };
            }
            Matcher::Template => {
                if !is_byte_in_str(s, toks) {
                    let remain = &toks[s..];

                    match remain.find("</temp>") {
                        Some(a) => {
                            return if !is_byte_in_str(a, toks) {
                                Some(Mp::new(remain[6..a].to_string(), s + 6, None))
                            } else {
                                collect_scope(remain, &Matcher::Template, i_s)
                            }
                        }
                        None => panic!("</temp> expected to end the template scope"),
                    }
                }

                collect_scope(&toks[s..], &Matcher::Template, i_s)
            }
            Matcher::Dom => collect_scope(toks, &Matcher::Component("dom"), i_s),
            Matcher::Sin => collect_scope(toks, &Matcher::Component("sin"), i_s),
            Matcher::Cam => collect_scope(toks, &Matcher::Component("cam"), i_s),
        },
        None => None,
    };
}
