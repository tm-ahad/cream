use crate::pass::pass;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::marker::PhantomData;

pub struct DspMap<'a>(BTreeMap<String, String>, &'a PhantomData<*const bool>);

impl<'a> DspMap<'a> {
    pub fn new() -> DspMap<'a> {
        DspMap(BTreeMap::new(), &PhantomData::<*const bool>)
    }

    pub fn load(&mut self, path: &str) {
        let cont = read_to_string(path).unwrap_or_else(|e| panic!("{e}"));

        let lines = cont.lines();

        for lin in lines {
            let (k, v) = match lin.split_once('$') {
                Some(a) => (String::from(a.0), String::from(a.1)),
                None => pass(),
            };

            self.0.insert(k, v);
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
