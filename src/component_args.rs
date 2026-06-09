use crate::{dsp_map::DspMap, import_base::ImportBase};

pub struct ComponentArgs {
    pub config: DspMap,
    pub import_base: ImportBase
}

impl ComponentArgs {
    pub fn new(config: DspMap, import_base: ImportBase) -> Self {
        ComponentArgs {
            config,
            import_base
        }
    }
}
