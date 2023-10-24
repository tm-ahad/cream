use crate::component::{component_call, component_call_len, Component};
use crate::helpers::dnl::dnl;
use crate::helpers::is_in_temp::is_in_temp;
use crate::helpers::script_in::{script_in_html, script_in_scope};
use std::collections::BTreeMap;

pub fn transpile_component(ccm: BTreeMap<u32, Component>, script: &mut String, html: &mut String) {
    for (id, c) in ccm.iter() {
        if let Some(idx) = html.find(&component_call(*id)) {
            let end = component_call_len(dnl(id));

            if is_in_temp(html, idx) {
                let scr = &script_in_html(&c.script, &c.dom_script);

                html.replace_range(idx..idx + end, &c.html.stat);
                script.insert_str(0, scr);
            } else {
                let s_scr = &script_in_scope(&c.dyn_script, &c.dom_script);

                html.replace_range(idx..idx + end, &c.html.dynamic);
                script.push_str(s_scr)
            }
        }
    }
}
