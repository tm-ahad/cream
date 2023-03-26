use std::fs::{self, File};
use std::io::Write;

pub fn new(name: &String) {
    fs::create_dir(format!("./{}", name)).expect("Creating dir not allowed");
    fs::create_dir(format!("./{}/src", name)).expect("Creating dir not allowed");
    fs::create_dir(format!("./{}/build", name)).expect("Cannot create dir!");

    fs::rename("./lib", format!("./{}/lib", name)).expect("Can't even move dir in ohio");
    fs::rename(format!("./{}/lib", name), "./lib").expect("Can't even move dir in ohio");

    let mut f = File::create(format!("./{}/src/app.js", name)).expect("Cannot create file");

    f.write(
        "
app {
    <html>
        <h1></h1>
    </html>
}"
        .as_bytes(),
    )
    .expect("Cannot write file");

    let _ = File::create(format!("./{}/dep_map.yaml", name)).expect("Creating file not permited");
}
