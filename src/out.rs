use crate::dsp_map::DspMap;
use std::fs;
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
    _config: &DspMap
) {
    write_file(path, &script)
        .unwrap_or_else(|e| panic!("{}", e));
}
