use crate::component::{component, Component};
use crate::component_args::ComponentArgs;
use crate::dsp_map::DspMap;
use crate::std_err::ErrType::NotFound;
use crate::std_err::StdErr;
use std::collections::btree_map::Keys;
use std::collections::BTreeMap;

pub type KeyComponentPair<'a> = (&'a str, &'a Component<'a>);

pub struct ComponentMap<'a> {
    catch: BTreeMap<&'a str, Component<'a>>,
    component_args: ComponentArgs<'a>,
    path_map: DspMap<'a>,
    all_cached: bool
}

impl<'a> ComponentMap<'a> {
    pub fn new(path_map: DspMap<'a>, component_args: ComponentArgs<'a>) -> Self {
        Self {
            catch: BTreeMap::new(),
            all_cached: false,
            component_args,
            path_map,
        }
    }

    fn get_(&mut self, key: &'a str, all_cached: bool) -> &Component {
        match self.catch.get(key) {
            Some(component) => component,
            None => {
                if all_cached {
                    StdErr::exec(NotFound, &format!("Couldn't find component {}", key));
                    todo!()
                } else {
                    let path = self.path_map.get(key).unwrap();
                    let comp = self.call_component(path, key);

                    self.catch.insert(key, comp);
                    self.catch.get(key).unwrap()
                }
            }
        }
    }

    pub fn get(&mut self, key: &'a str) -> &Component {
        self.get_(key, match self.all_cached {
            true => true,
            false => false,
        })
    }

    fn call_component(&mut self, path: &str, key: &str) -> Component<'a> {
        let comp = component::<'a>(
            path,
            key,
            self,
            self.component_args.transpile_command,
            self.component_args.config,
        );

        comp
    }

    pub fn keys(&'a self) -> Keys<'a, String, String> {
        self.path_map.keys()
    }
    pub fn kv_pairs(&'a mut self) -> Vec<KeyComponentPair<'a>> {
        let mut ret = Vec::new();
        let keys = self.path_map.keys().clone();

        for key in keys {
            ret.push((key.as_str(), self.get_(key, false)));
        }

        self.all_cached = true;
        ret
    }
}
