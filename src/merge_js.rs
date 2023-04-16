use crate::get_prop::get_prop;
use std::collections::HashMap;
use std::fs::{File, read_to_string, write};
use std::io::Read;

pub fn merge_js(map: HashMap<String, String>) {
    let path = &*get_prop(map.clone(), "_app_html");
    let mut file = File::open(path)
                              .expect("File not found");

    let js = read_to_string(format!("./build/{}", get_prop(map.clone(), "_app_js"))
        ).expect("File not found");

    let mut content: String = String::new();
    file.read_to_string(&mut content)
        .expect("Cannot read file");

    let id = content.len() - 17;

    content.insert_str(id, &*format!("\n<script>\n{js}\n</script>"));

    write(path, content.as_bytes())
        .expect("Cannot write file");
}