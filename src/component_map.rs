use crate::component::{component, Component};
use crate::component_args::ComponentArgs;
use std::collections::BTreeMap;
use crate::dsp_map::DspMap;

pub struct ComponentMap<'a> {
    cache: BTreeMap<String, Component>,
    component_args: ComponentArgs<'a>
}

impl<'a> ComponentMap<'a> {
    pub fn new(component_args: ComponentArgs<'a>) -> Self {
        Self {
            cache: BTreeMap::new(),
            component_args
        }
    }

    pub fn get(&mut self, f_name: String, c_name: String) -> Component {
        if let Some(c) = self.cache.get(c_name.as_str()) {
            return c.clone();
        }

        let new_component = component(f_name, c_name.clone(), self);
        self.cache.insert(c_name.clone(), new_component);
        self.cache.get(c_name.as_str()).unwrap().clone()
    }


    pub fn config(&self) -> &'a DspMap {
        self.component_args.config
    }
}
