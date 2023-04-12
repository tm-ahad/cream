use std::error::Error;
use crate::copy_dir::copy;
use crate::pass::pass;
use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use crate::input::std_input;
use std::fs::{File, create_dir};
use std::io::Write;

pub fn moving_denied(e: Box<dyn Error>) {
    let err = StdErr::new(OSError,
                          &*e.to_string());

    err.exec();
}

pub fn new(name: &String) {
    let lang = std_input("Language for the project", "ts");

    match create_dir(format!("./{}", name)) {
        Ok(_) => pass(),
        Err(e) => {
            let err = StdErr::new(OSError, &*
                e.to_string());

            err.exec();
        }
    };

    create_dir(format!("./{}/src", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/build", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/lib", name)).expect("Creating dir not allowed");

    match copy("./build/node", format!("./{}/build/", name)) {
        Ok(_) => pass(),
        Err(a) => moving_denied(Box::new(a))
    };

    match copy(format!("./lib/{lang}"), format!("./{}/lib", name)) {
        Ok(_) => pass(),
        Err(a) => moving_denied(Box::new(a))
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
