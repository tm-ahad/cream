use std::env::consts::OS;
use crate::input::std_input;
use std::fs::{File, create_dir};
use std::io::Write;

pub fn new(name: &String) {
    let input = std_input("Language for your project (ts): ", "ts");
    let k = std_input("keywords: ", "");
    let a = std_input("description: ", "");
    let t = std_input("author: ", "");
    let d = std_input("title: ", "");
    let n = std_input(&*format!("name ({name}): "), name);

    let sh = match OS {
        "win32" => ".bat",
        "linux" | "darwin" => ".sh",
        _ => ""
    };

    create_dir(format!("./{}", name)).expect("Directory Exists");
    create_dir(format!("./{}/src", name)).expect("Directory Exists");
    create_dir(format!("./{}/build", name)).expect("Directory Exists");

    let mut f = File::create(format!("./{}/src/app.{input}", name))
        .expect("File exists");

    let mut config = File::create(format!("./{}/config.dsp", name))
        .expect("File exists");

    let mut shell = File::create(format!("./{}/start{sh}", name))
        .expect("File exists");

    shell.write("\
nts build
serve ./build/index.html 127.0.0.1:8871\n"
        .as_bytes()).expect("Cannot write file");

    config.write(format!("\
name${n}
lang${input}
head$
pre_build$
pre_start$
keywords${k}
author${a}
description${d}
title${t}
port$8871
host$127.0.0.1
_app_js$build/app.js
_app_html$build/index.html").as_bytes())
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
