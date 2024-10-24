use crate::dsp_map::DspMap;

pub struct ComponentArgs<'a> {
    pub config: &'a DspMap<'a>,     //Config map
}

impl<'a> ComponentArgs<'a> {
    pub fn new(config: &'a DspMap<'a>) -> Self {
        ComponentArgs {
            config
        }
    }
}
