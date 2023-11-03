use crate::consts::{SBF, SBS};

pub fn parse_dyn_component(script: &str, html: &str) -> String {
    format!("(() => {SBF}{script} return `{html}`\n{SBS}())")
}

pub fn parse_stat_component_script(dom_script: &str) -> String {
    format!("{SBF}{dom_script}{SBS}")
}
