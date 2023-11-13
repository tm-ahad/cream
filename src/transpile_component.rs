use crate::component::{component_call, component_call_len, Component};
use crate::helpers::is_in_temp::is_in_temp;
use crate::helpers::script_in::{parse_dyn_component, parse_stat_component_script};
use crate::helpers::javascript_string::javascript_string;
use crate::component_markup::ComponentMarkUp;
use crate::helpers::dnl::dnl;
use crate::helpers::to_raw_parsable_format::to_raw_parsable_format;
use std::collections::BTreeMap;

pub fn transpile_component(
    ccm: BTreeMap<u32, Component>,
    script: &mut String,
    html: &mut ComponentMarkUp
) {
    let ps = to_raw_parsable_format(&*script, &html.stat);

    for (id, c) in ccm.iter() {
        if let Some(idx) = ps.raw.find(&component_call(*id)) {
            let end = component_call_len(dnl(id));
            let is_static = is_in_temp(&ps.raw, idx);

            if is_static {
                let scr = &parse_stat_component_script(&c.script, &c.dom_script);
                let s = &javascript_string(&c.html.stat);
                let imo = idx-ps.temp_starts;

                html.stat.replace_range(imo..imo + end, &c.html.stat);
                html.dynamic.replace_range(imo..imo + end,  s);
                script.insert_str(0, scr);
            } else {
                let s_scr = &parse_dyn_component(&c.dyn_script, &c.html.dynamic);
                
                script.replace_range(idx-1..idx + end, s_scr);
            }
        }
    }
}
