use std::collections::HashMap;

struct Rel(pub String, pub String);

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

    pub fn parse(&mut self, key: &str, v: String, ext: &str) -> String {
        let val = self.map.get_mut(key);
        let mut rels = Vec::new();
        let mut p = format!("{key}={v}\n");

        match val {
            Some(l) => {
                if l.1.is_empty() {
                    for (k, val) in &l.0 {

                        rels.push(Rel(
                            k.to_string(),
                            val.to_string()
                        ));
                    }
                } else {
                    l.1 = format!("{}={}{}\n", key, v , ext);
                    return l.1.clone();
                }
            }
            None => return String::new(),
        }

        for rel in rels {
            let key = rel.0;
            let val = rel.1;

            let fmt = &format!("{}={}{}\n", key, val, ext);
            p.push_str(fmt);

            p.push_str(&self.parse(&key, val, ext));
        }

        p
    }
}
