use crate::component::{component_call, Component};
use crate::component_markup::ComponentMarkUp;
use crate::helpers::component_part::ComponentPart;
use crate::helpers::read_until::read_until;
use crate::id_gen::IdGen;
use std::collections::BTreeMap;

pub fn extract_component(
    ccm: &mut BTreeMap<u32, Component>,
    imports: &Vec<Component>,
    cmu: &mut ComponentMarkUp,
    f_name: &str
) {
    for comp in imports {
        let m = &format!("<{}", comp.name);

        if let Some(e) = cmu.stat.find(m) {
            let cde = read_until(&cmu.stat, e+comp.name.len(), "/>", f_name, ComponentPart::Unknown);
            let id = IdGen::gen_u32();
            let cl = comp.clone();

            ccm.insert(id, cl);

            cmu.dynamic.replace_range(e..cde + 2, &component_call(id));
            cmu.stat.replace_range(e..cde + 2, &component_call(id));
        }
    }
}
