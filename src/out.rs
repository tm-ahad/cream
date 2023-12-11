use crate::consts::{NEW_LINE, NIL};
use crate::dsp_map::DspMap;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

pub fn out(
    path: &str,
    html: String,
    script: String,
    config: &DspMap
) {
    let head_prefix = format!("./{}", config.expect("head_prefix"));
    let head = read_to_string(head_prefix.clone())
        .unwrap_or_else(|e| panic!("{head_prefix}: {e}"));

    let html = html.replace(NEW_LINE, NIL);

    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .unwrap_or_else(|e| panic!("{}", e));

    file.write_all(
        format!(
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
{html}
<script>
{script}
</script>
<body>
</html>
",
            config.expect("description"),
            config.expect("keywords"),
            config.expect("author"),
            config.expect("title")
        )
        .as_bytes(),
    )
    .unwrap_or_else(|e| panic!("{}", e));
}
