use crate::consts::NOT_FOUND_RESPONSE_FILE;
use crate::dsp_map::DspMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn out(path: &str, html: String, script: String, config: &DspMap) {
    let head = config.expect("head");

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

pub fn out_to_error(error_page: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(NOT_FOUND_RESPONSE_FILE)
        .unwrap_or_else(|e| panic!("{}", e));

    file.write_all(error_page.as_bytes())
        .unwrap_or_else(|e| panic!("{}", e));
}
