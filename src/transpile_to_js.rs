use crate::consts::BUILT_JS;
use crate::sys_exec::sys_exec;
use std::fs::{read_to_string, write};

pub fn transpile_script(lang: &str, transpile_command: &str, script: &mut String) {
    write(format!("./build/.$.{lang}"), script.clone())
        .unwrap_or_else(|e| panic!("{}", e));
    sys_exec(format!("{transpile_command} ./build/.$.{lang}"));
    let res = read_to_string(BUILT_JS);

    if let Ok(s) = res {
        *script = s;
    }
}
