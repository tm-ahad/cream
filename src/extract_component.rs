use crate::component::{component_call, Component};
use crate::id_gen::IdGen;
use std::collections::BTreeMap;

pub fn extract_component(
    ccm: &mut BTreeMap<u32, Component>,
    imports: &Vec<Component>,
    script: &mut String,
    html: &mut String,
) {
    for comp in imports {
        let m = &format!("<{}", comp.name);

        if let Some(e) = html.find(m) {
            let mut cde = e + comp.name.len();

            while &html[cde..cde + 2] != "/>" {
                cde += 1;
            }

            let id = IdGen::gen_u32();
            let cl = comp.clone();

            ccm.insert(id, cl);
            html.replace_range(e..cde + 2, &component_call(id));

            script.push('\n');
            script.push_str(&comp.script)
        }
    }
}
