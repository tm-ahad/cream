use crate::input::std_input;
use std::fs::{File, create_dir};
use std::io::Write;

pub fn new(name: &String) {
    let input = std_input("Language for your project (ts): ", "ts");
    let k = std_input("keywords: ", "ts");
    let a = std_input("description: ", "ts");
    let t = std_input("author: ", "ts");
    let d = std_input("title: ", "ts");
    let n = std_input("name (ex): ", "ex");

    create_dir(format!("./{}", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/src", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/build", name)).expect("Creating dir not allowed");
    create_dir(format!("./{}/lib", name)).expect("Creating dir not allowed");

    let mut f = File::create(format!("./{}/src/app.{input}", name))
        .expect("Cannot create file");

    let mut config = File::create(format!("./{}/config.dsp", name))
        .expect("Cannot create file");

    config.write(format!("\
name${n}
lang${input}
pre_build$
pre_start$
keywords${k}
author${a}
description${d}
title${t}
port$8871
host$127.0.0.1
app$app.{input}").as_bytes())
        .expect("Cannot write file");

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
}
