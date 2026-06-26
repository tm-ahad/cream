mod component;
mod consts;
mod dsp_map;
mod helpers;
mod input;
mod javascript_lib;
mod router;
mod transpiler;
mod std_err;
mod new;
mod out;
mod pass;
mod serve;

use crate::dsp_map::DspMap;
use crate::consts::{CONFIG_FILE};
use crate::helpers::version::version;
use crate::serve::serve;
use crate::new::new;
use crate::pass::pass;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        let ne = "cream new {project_name}: Create a new project";
        let build = "cream make: Build your project";
        let serve = "cream serve: Serve or run your project";
        let vers = "cream version: Output's cream's version that is currently installed";

        let inst = format!("{ne}\n{build}\n{serve}\n{vers}");
        println!("{inst}");
    } else {
        let mut map;

        match args[1].as_str() {
            "new" => new(args.get(2).expect("Project name not provided")),
            "make" => {
                map = DspMap::new();
                map.load(CONFIG_FILE);
                router::router(&mut map);
            },
            "serve" => {
                map = DspMap::new();
                map.load(CONFIG_FILE);
                serve(map)
            },
            "version" => println!("{}", version()),
            &_ => pass(),
        }
    }
}
