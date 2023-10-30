use crate::component::{component_call, component_call_len, Component};
use crate::helpers::is_in_temp::is_in_temp;
use crate::helpers::script_in::{script_in_html, script_in_scope};
use crate::parsable_format::ParsableFormat;
use crate::helpers::dnl::dnl;
use std::collections::BTreeMap;

pub fn transpile_component(
    ccm: BTreeMap<u32, Component>,
    script: &mut String,
    html: &mut String,
    ps: ParsableFormat
) {
    for (id, c) in ccm.iter() {
        if let Some(idx) = ps.raw.find(&component_call(*id)) {
            let end = component_call_len(dnl(id));

            if is_in_temp(&ps.raw, idx) {
                let scr = &script_in_scope(&c.script, &c.dom_script);
                let imo = idx-ps.temp_starts;

                html.replace_range(imo..imo + end, &c.html.stat);
                script.insert_str(0, scr);
            } else {
                let s_scr = &script_in_html(&c.dyn_script, &c.dom_script);

                script.replace_range(idx..idx + end, &format!(
                    "{}{}",
                    s_scr,
                    &c.html.dynamic
                ));
            }
        }
    }
}
