#[derive(Debug)]
pub struct _StateBase {
    pub map: Vec<(String, Vec<String>)>,
    pub parse: String,
}

impl _StateBase {
    pub fn new() -> _StateBase {
        _StateBase {
            map: vec![],
            parse: String::new(),
        }
    }

    pub fn _set(&mut self, k: String, v: String) {
        if k != v {
            let mut p_a: bool = false;

            for l in &mut self.map {
                if l.0 == k {
                    p_a = true;
                    l.1.push(v.clone());
                }
            }

            if !p_a {
                self.map.push((k, vec![v]))
            }
        }
    }

    pub fn parse(&mut self, key: String, ext: String) {
        for l in &self.map {
            if l.0 == key {
                let mut p = format!("function update{}(v) ", l.0);
                p.push_str("{\n   ");
                p.push_str(format!("{}=v\n{}", l.0, ext).as_str());

                for vls in &l.1 {
                    p.push_str(format!("   {}=v\n{}", vls, ext).as_str())
                }
                p.push_str("}");

                self.parse = p
            }
        }
    }
}
