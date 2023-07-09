use crate::brace_pool::BracePool;
use crate::sys_exec::sys_exec;
use crate::channel::{Channel, Input};
use crate::is_byte_in_str::is_byte_in_str;
use crate::mp::Mp;

pub fn collect_scope(toks: &String, matchr: &String) -> Option<(String, usize)> {
    return match toks.find(matchr) {
        Some(s) => {
            let ss = s;
            let remain = &toks[s+matchr.len()..];

            return match remain.find('{') {
                Some(ss) => {
                    let dif = &remain[..ss];

                    if dif.trim().is_empty() {
                        let mut pool = BracePool::new();
                        let i = ss+1;

                        while &remain[i..i+1] != "{" {
                            pool.push('{');
                        }

                        while &remain[i..i+1] != "}" {
                            if pool.push('}') && !is_byte_in_str(i, &remain[ss+1..i]) {
                                Some((remain[ss+1..i].to_string(), s+i));
                            };
                        }
                    } else {
                        collect_scope(&toks[s+ss..].to_string(), matchr)
                    }
                },
                None => collect_scope(&remain.to_string(), matchr)
            }
        }
        None => None,
    }

}
