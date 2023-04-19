use crate::input::std_input;
use crate::std_out::std_out;
use std::env::consts::OS;
use std::fs::{File, create_dir};
use std::io::Write;
use colored::Colorize;

pub fn new(name: &String) {
    let input = std_input("Language for your project (ts): ", "ts");
    let k = std_input("keywords: ", "");
    let a = std_input("description: ", "");
    let t = std_input("author: ", "");
    let d = std_input("title: ", "");
    let n = std_input(&*format!("name ({name}): "), name);

    let ok = std_input("Ok to processed (y)?", "y");

    if ok == "y" {
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
serve"
            .as_bytes()).expect("Cannot write file");

        config.write(format!("\
home$/
static_dir$
static_dir_render$
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

        std_out(&*format!("{} ✨\n", "Done".green().bold()));
    }

}
