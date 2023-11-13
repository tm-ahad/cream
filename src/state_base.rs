use crate::helpers::rand_hex::rand_hex;
use std::collections::HashMap;

struct Rel(pub String, pub String, pub String);

pub struct _StateBase {
    pub map: HashMap<String, (HashMap<String, String>, String)>,
}

impl _StateBase {
    pub fn new() -> _StateBase {
        _StateBase {
            map: HashMap::new(),
        }
    }

    pub fn _set(&mut self, k: String, v: String, rb: String) {
        if k != v {
            match self.map.get_mut(&k) {
                Some(val) => {
                    val.0.insert(v, rb);
                }
                None => {
                    let mut map = HashMap::new();
                    map.insert(v, rb);

                    self.map.insert(k, (map, String::new()));
                }
            }
        }
    }

    pub fn parse(&mut self, key: &String, ext: String, v: String) -> String {
        let val = self.map.get_mut(key);
        let mut rels = Vec::new();
        let mut p = String::new();

        let mut vn = String::new();

        match val {
            Some(l) => {
                if l.1.is_empty() {
                    for (k, val) in &l.0 {
                        if k.trim() == key {
                            continue;
                        }

                        vn = rand_hex();

                        rels.push(Rel(
                            k.clone(),
                            ext.clone(),
                            val.clone()
                        ));
                    }
                } else {
                    l.1 = format!("{}={}{}\n", key, v, ext);
                    return l.1.clone();
                }
            }
            None => return vn,
        }

        for rel in rels {
            let key = rel.0;
            let val = rel.1;
            let ext = rel.2;

            let fmt = &format!("{}={}{}\n", key, vn, ext);
            p.push_str(fmt);

            p.push_str(&self.parse(&key, val, ext));
        }



        format!("var {vn} = {v}\n{}", p)
    }
}
