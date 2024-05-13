use crate::input::std_input;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::consts::NIL;
use crate::helpers::create_file::create_file;
use crate::helpers::format_colored::format_colored;
use std::fs::{create_dir, File};
use std::io::Write;

pub fn new(name: &String) {
    let lang = std_input("Language for your project (js/ts): ", "js");
    let k = std_input("keywords: ", NIL);
    let a = std_input("description: ", NIL);
    let t = std_input("author: ", NIL);
    let d = std_input("title: ", NIL);
    let n = std_input(&format!("name ({name}): "), name);

    let ok = std_input("Ok to processed (y)? ", "y");

    if ok == "y" || ok == "yes" || ok == "ok" {
        create_dir(format!("./{}", name)).expect("Directory Exists");
        create_dir(format!("./{}/src", name)).expect("Directory Exists");
        create_dir(format!("./{}/build", name)).expect("Directory Exists");

        let mut f = File::create(format!(
            "./{}/src/app.{lang}",
            name,
        ))
        .expect("File exists");

        let mut config = File::create(format!("./{}/config.dsp", name)).expect("File exists");
        let (build, inst) = if lang == "ts" {
            (
                "npx tsc",
                format_colored("Now setup typescript with node js", 255, 204, 000)
            )
        } else {
            (NIL, format!("{} ✨", format_colored("Done", 0, 255, 0)))
        };

        create_file(format!("./{}/build/error.html", name));
        create_file(format!("./{}/build/index.html", name));
        create_file(format!("./{}/head_prefix.html", name));

        create_file(format!("./{}/build/.$.js", name));
        create_file(format!("./{}/build/.$.ts", name));

        create_file(format!("./{}/routes.json", name));

        config
            .write_all(
                format!(
                    "\
routes$src/routes.json
static_dir$
static_dir_render$
name${n}
lang${lang}
head_prefix$head_prefix.html
pre_make$
build${build}
pre_start$
keywords${k}
author${a}
description${d}
title${t}
port$8871
host$127.0.0.1
_app_html$build/index.html"
                )
                .as_bytes(),
            )
            .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));

        f.write_all(
            "
app {
    <temp>
        <h1>Hello World</h1>
    </temp>
}\n"
            .as_bytes(),
        )
        .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));


        println!("{inst}");
    }
}
