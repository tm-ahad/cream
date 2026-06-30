mod component;
mod consts;
mod config;
mod helpers;
mod input;
mod javascript_lib;
mod make;
mod transpiler;
mod std_err;
mod create_project;
mod out;
mod pass;
mod serve;

use notify::{Event, EventKind, RecursiveMode, Watcher};
use crate::consts::{CONFIG_FILE};
use crate::helpers::version::version;
use crate::create_project::create_project;
use crate::pass::pass;
use crate::serve::serve;
use crate::std_err::ErrType::{NotFound, SyntaxError};
use crate::std_err::StdErr;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        let ne = "cream new {project_name}: Create a new project";
        let build = "cream make: Build your project";
        let serve = "cream serve: Serve or run your project";
        let vers = "cream version: Output's cream's version that is currently installed";

        let inst = format!("{ne}\n{build}\n{serve}\n{vers}");
        println!("{inst}");
    } else {
        let mut config;

        match args[1].as_str() {
            "new" => create_project(args.get(2).expect("Project name not provided")),
            "make" | "serve" => {
                config = toml::from_str(
                    &read_to_string(CONFIG_FILE)
                        .unwrap_or_else(|_| {
                            StdErr::exec(NotFound, CONFIG_FILE);
                            exit(1)
                        })
                ).unwrap_or_else(|e| {
                    StdErr::exec(
                        SyntaxError,
                        &format!("config file contains invalid toml: {}", e)
                    );
                    exit(1)
                });

                if !cfg!(debug_assertions) || args[1].as_str() == "make" {
                    make::make(&mut config);
                }

                if args[1].as_str() == "serve" {
                    let bind_conf = config.clone();
                    let mut watcher = notify::recommended_watcher
                        (move |event: Result<Event, notify::Error>| {
                            if let Ok(event) = event {
                                match event.kind {
                                    EventKind::Create(_) |
                                    EventKind::Remove(_) |
                                    EventKind::Modify(_) => {
                                        std::thread::sleep(
                                            std::time::Duration::from_millis(300)
                                        );
                                        println!("Building...");
                                        make::make(&bind_conf);
                                    },
                                    _ => {}
                                }
                            }
                        }).unwrap();

                    let _ = watcher.watch(
                        Path::new("./src").as_ref(),
                        RecursiveMode::Recursive
                    );

                    let bind_conf = config.clone();
                    serve(&bind_conf).await
                }
            },
            "version" => println!("{}", version()),
            &_ => pass(),
        }
    }
}
