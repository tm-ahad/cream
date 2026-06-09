use crate::component::Component;
use crate::component_args::ComponentArgs;
use crate::transpiler::transpile_component_;
use std::collections::BTreeMap;
use crate::dsp_map::DspMap;

pub struct ComponentMap {
    cache: BTreeMap<String, Component>,
    pub component_args: ComponentArgs
}

impl ComponentMap {
    pub fn new(component_args: ComponentArgs) -> Self {
        Self {
            cache: BTreeMap::new(),
            component_args
        }
    }

    pub fn get(&mut self, f_name: String, c_name: String) -> Component {
        if let Some(c) = self.cache.get(c_name.as_str()) {
            return c.clone();
        }

        let new_component = transpile_component_(
            &mut self.component_args.import_base,
            &self.component_args.config,
            f_name,
            c_name.clone()
        );
        
        self.cache.insert(c_name.clone(), new_component);
        self.cache.get(c_name.as_str()).unwrap().clone()
    }


    pub fn config(&self) -> DspMap {
        self.component_args.config.clone()
    }
}
