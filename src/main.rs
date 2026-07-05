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
use crate::helpers::check_sums::CheckSums;
use crate::consts::{CHECK_SUM_FILE, CONFIG_FILE};
use crate::helpers::version::version;
use crate::create_project::create_project;
use crate::pass::pass;
use crate::serve::serve;
use crate::std_err::ErrType::{NotFound, SyntaxError};
use crate::std_err::StdErr;
use std::collections::{BTreeMap};
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use std::env;

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
                    let raw_check_sum = read_to_string(CHECK_SUM_FILE)
                        .unwrap_or(String::new());

                    let mut check_sums =
                         serde_json::from_str::<CheckSums>(&raw_check_sum)
                            .unwrap_or(BTreeMap::new());

                    make::make(&mut config, &None, &mut check_sums);
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
                                        make::make(
                                            &bind_conf,
                                            &Some(event),
                                            &mut BTreeMap::new()
                                        );
                                    },
                                    _ => {}
                                }
                            }
                        }).unwrap();

                    let _ = watcher.watch(
                        Path::new("./src").as_ref(),
                        RecursiveMode::Recursive
                    );
                    let _ = watcher.watch(
                        Path::new(CONFIG_FILE).as_ref(),
                        RecursiveMode::NonRecursive
                    );

                    let bind_conf = config.clone();
                    serve(&bind_conf).await
                }
            },
            "version" => println!("Cream {}", version()),
            &_ => pass(),
        }
    }
}
