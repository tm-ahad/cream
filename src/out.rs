use crate::component::{Component, cream_dom_name};
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
    html: String,
    script: String,
    config: &DspMap
) {
    let head_prefix = format!("./{}", config.expect("head_prefix"));
    let head = read_to_string(head_prefix.clone())
        .unwrap_or_else(|e| panic!("{head_prefix}: {e}"));

    let comp = Component::new(String::new(), html, String::new(), String::new());
    let (html, id) = comp.html_rendering_script().unwrap();

    write_file(&path,
        &format!(
            "
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta name=\"description\" content=\"{}\">
    <meta name=\"keywords\" content=\"{}\">
    <meta name=\"author\" content=\"{}\">
    <title>{}</title>
    {head}
</head>
<body>
<script>
var self;
{script};
{html};
document.body.appendChild({})
</script>
<body>
</html>
",
            config.expect("description"),
            config.expect("keywords"),
            config.expect("author"),
            config.expect("title"),
            cream_dom_name(id)
        ),
    )
    .unwrap_or_else(|e| panic!("{}", e));
}
