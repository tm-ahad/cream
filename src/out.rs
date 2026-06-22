use crate::dsp_map::DspMap;
use std::fs::{self, read_to_string};
use std::path::Path;


fn write_file(path: &str, contents: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;
    Ok(())
}

pub fn out(
    path: &str,
    script: String,
    config: &DspMap
) {
    let head_prefix = format!("./{}", config.expect("head_prefix"));
    let head = read_to_string(head_prefix.clone())
        .unwrap_or_else(|e| panic!("{head_prefix}: {e}"));

    write_file(&path, &script)
    .unwrap_or_else(|e| panic!("{}", e));
}
