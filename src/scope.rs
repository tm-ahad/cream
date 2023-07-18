use crate::collect_scope::collect_scope;
use crate::config::Config;
use crate::matcher::Matcher;
use crate::sys_exec::sys_exec;
use std::collections::HashMap;
use std::fs::{read_to_string, write};

pub fn parse_scope(script: &mut String, ptr: &mut HashMap<usize, String>) {
    let mut indexes = (0, 0, 0);
    let matchers: [Matcher; 3] = [Matcher::Dom, Matcher::Cam, Matcher::Sin];

    for m in matchers {

        while let Some(pat) = collect_scope(script, &m) {
            let ind = pat.index();
            indexes.0 += ind;

            ptr.insert(indexes.0, pat.mp().clone());
            script.replace_range(ind..ind + pat.mp_len(), "");
        }
    }
}

pub fn scopify(script: &mut String, map: HashMap<usize, String>, config: &Config) {
    for (k, v) in map {
        write(format!("./build/.$.{}", config.expect("lang")), &v)
            .unwrap_or_else(|e| panic!("{:?}", e));

        sys_exec(format!(
            "{} ./build/.$.{}",
            config.expect("build"),
            config.expect("lang")
        ));
        let res = read_to_string("./build/.$.js").unwrap();

        script.insert_str(k, &res);
    }
}
