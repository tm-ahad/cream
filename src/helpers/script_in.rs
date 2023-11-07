use crate::consts::{SBF, SBS};

pub fn parse_dyn_component(script: &str, html: &str) -> String {
    format!("(() => {SBF}{script} return `{html}`\n{SBS})()")
}

pub fn parse_stat_component_script(script: &str, dom_script: &str) -> String {
    format!("{SBF}{script}\n{dom_script}{SBS}")
}
