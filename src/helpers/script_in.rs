use crate::consts::{SBF, SBS};

pub fn script_in_html(script: &str, dom_script: &str) -> String {
    format!("\n${SBF}(() => {SBF}{script}{dom_script}return \"\"\n{SBS})(){SBS}\n")
}

pub fn script_in_scope(script: &str, dom_script: &str) -> String {
    format!("{SBF}{script}\n{dom_script}{SBS}")
}
