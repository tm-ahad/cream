use std::collections::HashMap;
use rusty_v8::{ContextScope, HandleScope, Script};
use crate::v8_parse::v8_parse;

#[derive(Debug)]
pub struct _StateBase {
    pub map: HashMap<String, (HashMap<String, String>, String)>,
    pub parse: String,
}

impl _StateBase {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            parse: String::new(),
        }
    }

    pub fn _set(&mut self, k: String, v: String, rb: String) {
        if k != v {
            let mut p_a: bool = false;

            for l in &mut self.map {
                if l.0 == &k {
                    p_a = true;
                    l.1.0.insert(v.clone(), rb.clone());
                }
                if l.0 == &k {
                    p_a = true;
                    l.1.0.insert(v.clone(), rb.clone());
                }
            }

            if !p_a {
                let mut map = HashMap::new();
                map.insert(v, rb);

                self.map.insert(k, (map, String::new()));
            }
        }
    }
    
    pub fn parse(
                &mut self,
                key: String,
                ext: String,
                v: String,
                scope: &mut ContextScope<HandleScope>
            ) {
                let val = self.map.get_mut(&*key);

                match val {
                    Some(l) => {
                        if l.1 == String::new() {
                            let mut p = String::new();

                            for (k, vl) in &l.0 {
                                let result = v8_parse(scope, vl);
                                let check = v8_parse(scope, k);

                                if result != check {
                                    let fmt = &*format!("{}={}{}\n", k, result, ext);
                                    let v8_str = rusty_v8::String::new(scope, fmt).unwrap();

                                    p.push_str(fmt);

                                    let s = Script::compile(scope, v8_str, None);
                                    let _ = s
                                        .unwrap()
                                        .run(scope);
                                }
                            }

                            self.parse = p.clone();
                            l.1 = p.clone();
                        } else {
                            l.1 = format!("   {}={}{}\n", key, v, ext)
                        }
                    }
                    None => {}
                }
            }
}
