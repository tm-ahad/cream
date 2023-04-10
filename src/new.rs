use crate::copy_dir::copy;
use crate::pass::pass;
use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use std::fs::{File, create_dir};
use std::io::Write;

pub fn moving_denied() {
    let err = StdErr::new(OSError,
                          "Permission denied to move dir");

    err.exec();
}

pub fn new(name: &String) {
    create_dir(format!("./{}", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/src", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/build", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/lib", name)).expect("Creating dir not allowed");

    match copy("./build/node", format!("./{}/build", name)) {
        Ok(_) => pass(),
        Err(_) => moving_denied()
    };

    match copy("./lib", format!("./{}/lib", name)) {
        Ok(_) => pass(),
        Err(_) => moving_denied()
    }

    let mut f = File::create(format!("./{}/src/app.js", name)).expect("Cannot create file");

    f.write(
        "
app {
    <html>
        <h1>Hello World</h1>
    </html>
}"
        .as_bytes(),
    )
    .expect("Cannot write file");

    let _ = File::create(format!("./{}/dep_map.yaml", name)).expect("Creating file not permited");
}
