use crate::component::{component_call, Component};
use crate::component_markup::ComponentMarkUp;
use crate::id_gen::IdGen;
use std::collections::BTreeMap;

pub fn extract_component(
    ccm: &mut BTreeMap<u32, Component>,
    imports: &Vec<Component>,
    cmu: &mut ComponentMarkUp,
) {
    for comp in imports {
        let m = &format!("<{}", comp.name);

        if let Some(e) = cmu.stat.find(m) {
            let mut cde = e + comp.name.len();

            while &cmu.stat[cde..cde + 2] != "/>" {
                cde += 1;
            }

            let id = IdGen::gen_u32();
            let cl = comp.clone();

            ccm.insert(id, cl);

            cmu.dynamic.replace_range(e..cde + 2, &component_call(id));
            cmu.stat.replace_range(e..cde + 2, &component_call(id));
        }
    }
}
