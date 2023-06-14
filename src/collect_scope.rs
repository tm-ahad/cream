use std::process::Command;

use crate::mp::Mp;

pub fn collect_scope(toks: &String, matchr: &String) -> (String, usize) {
    let mut com = Command::new("node");

    let output = com
        .arg("/home/ahad/.cream/tools/regexp/main.js")
        .arg(toks)
        .arg(matchr)
        .output()
        .expect("Failed to execute command");

    let bytes = output.stdout;

    let mut res = String::new();

    for byte in bytes {
        res.push(byte as char);
    }

    let p = Mp::parse(res);

    match p {
        Some((s, idx)) => 
            {
                let res = (*s).to_string();
                let start = res.find('{').unwrap();
                let mut id = start;

                while &res[id..id + 1] != "}" {
                    id += 1;
                }

                (res[start..id].to_string(), idx)
            },
        None => (String::new(), 0),
    }
}
