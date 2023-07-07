use crate::brace_pool::BracePool;
use crate::sys_exec::sys_exec;
use crate::channel::{Channel, Input};
use crate::mp::Mp;

pub fn collect_scope(toks: &String, matchr: &String) -> Option<(String, usize)> {
    return match toks.find(matchr) {
        Some(s) => {
            let ss = s;
            let remain = &toks[s..];

            match remain.find('{') {
                Some(ss) => {
                    let dif = &toks[s..s+ss];
                    let pool = BracePool::new();

                    while let Some(i) = dif.find("{") {
                        pool.push('}');
                        let dif = &dif[i..];

                        while let Some(i) = dif.find("}") {
                            if pool.push('}'); {

                            }
                        }
                    }


                },
                None => {}
            }
        }
        None => None,
    }

}
