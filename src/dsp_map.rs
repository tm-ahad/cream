use crate::pass::pass;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::marker::PhantomData;

//PhantomData<&'a *const bool> has no uses it's just fancy ✨
pub struct DspMap<'a>(BTreeMap<String, String>, PhantomData<&'a *const bool>);

impl<'a> DspMap<'a> {
    pub fn new() -> DspMap<'a> {
        DspMap(BTreeMap::new(), PhantomData::<&*const bool>)
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
