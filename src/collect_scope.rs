use crate::sys_exec::sys_exec;
use crate::channel::{Channel, Input};
use crate::mp::Mp;

pub fn collect_scope(toks: &String, matchr: &String) -> (String, usize) {
    let mut chan = Channel::new(String::from("./build/mp.chan"));

    chan.write(Input(matchr, toks));
    sys_exec(format!("node /home/ahad/.cream/tools/regexp/main.js"));

    let res = chan.read();
    let p = Mp::decode_res(res);

    match p {
        Some((s, idx)) => 
            {
                let res = (*s).to_string();
                let start = res.find('{').unwrap();
                let mut id = start;

                while &res[id..id + 1] != "}" {
                    id += 1;
                }
                
                (String::from(&res[start+1..id]), idx)
            },
        None => (String::new(), 0),
    }
}
