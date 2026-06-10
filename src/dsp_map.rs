use crate::pass::pass;
use std::collections::BTreeMap;
use std::fs::read_to_string;

//PhantomData<&'a *const bool> has no uses it's just fancy ✨
pub struct DspMap(BTreeMap<String, String>);

impl DspMap {
    pub fn new() -> DspMap {
        DspMap(BTreeMap::new())
    }

    pub fn load(&mut self, path: &str) {
        let cont = read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let lines = cont.lines();

        for lin in lines {
            let k;
            let v;

            match lin.split_once('$') {
                Some(a) => {
                    k = String::from(a.0);
                    v = String::from(a.1);

                    self.0.insert(k, v);
                },
                None => pass(),
            };
        }
    }

    pub fn get(&self, prop: &str) -> Option<&str> {
        match self.0.get(prop) {
            Some(v) => Some(v),
            None => None,
        }
    }

    pub fn expect(&self, prop: &str) -> &String {
        self.0
            .get(prop)
            .unwrap_or_else(|| panic!("Property {prop} not found on configuration"))
    }
}

impl Clone for DspMap {
    fn clone(&self) -> Self {
        DspMap(self.0.clone())
    }
}
