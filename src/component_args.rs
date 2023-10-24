use crate::dsp_map::DspMap;

pub struct ComponentArgs<'a> {
    pub transpile_command: &'a str, //transpile_command
    pub config: &'a DspMap<'a>,     //Config map
}


impl<'a> ComponentArgs<'a> {
    pub fn new(transpile_command: &'a str, config: &'a DspMap<'a>) -> Self {
        ComponentArgs {
            transpile_command,
            config
        }
    }
}
