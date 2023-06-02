use crate::input::std_input;
use crate::std_err::{StdErr, ErrType::OSError};
use crate::std_out::std_out;
use std::env::consts::OS;
use std::fs::{File, create_dir};
use std::io::Write;
use colored::Colorize;

pub fn new(name: &String) {
    let input = std_input("Language for your project (nts): ", "nts");
    let k = std_input("keywords: ", "");
    let a = std_input("description: ", "");
    let t = std_input("author: ", "");
    let is_mod = std_input("Project type (common/module): ", "common");
    let d = std_input("title: ", "");
    let n = std_input(&format!("name ({name}): "), name);

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

        let mut f = File::create(format!("./{}/src/app.{input}{}", name, if is_mod == "mod" {
            ".mod"
        } else {""}))
            .expect("File exists");

        let mut config = File::create(format!("./{}/config.dsp", name))
            .expect("File exists");

        let mut shell = File::create(format!("./{}/start{sh}", name))
            .expect("File exists");

        File::create("./build/.$.js")
            .unwrap_or_else(|e| panic!("{e}"));

        File::create("./build/error.html")
            .unwrap_or_else(|e| panic!("{e}"));

        shell.write_all(b"
cream make
serve").expect("Cannot write file");

        config.write_all(format!("\
home$/
static_dir$
static_dir_render$
name${n}
lang${input}
head$
pre_make$
build$
pre_start$
keywords${k}
author${a}
description${d}
title${t}
port$8871
host$127.0.0.1
_app_html$build/index.html").as_bytes())
            .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));


        f.write_all(
            "
app {
    <html>
        <h1>Hello World</h1>
    </html>
}"
                .as_bytes(),
        )
        .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));

        std_out(&format!("{} ✨\n", "Done".green().bold()));
    }

}
