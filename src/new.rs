use std::fs::{self, File};

pub fn new(name: &String) {
    fs::create_dir(format!("./{}", name))
        .expect("Creating dir not allowed");

    fs::create_dir(format!("./{}/src", name))
        .expect("Creating dir not allowed");

    fs::create_dir(format!("./{}/build", name))
        .expect("Cannot create dir!");

    let _ = File::create(format!("./{}/src/app.js", name))
        .expect("Cannot create file");

    let _ = File::create(format!("./{}/dep_map.yaml", name))
        .expect("Creating file not permited");


}