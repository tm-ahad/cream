mod collect_scope;
mod component;
mod component_args;
mod consts;
mod dsp_map;
mod helpers;
mod import_base;
mod import_lib;
mod import_npm;
mod import_script;
mod input;
mod javascript_lib;
mod matcher;
mod router;
mod script_module;
mod std_err;
mod transpiler;
mod import_template;
mod import_component;
mod component_map;
mod import_ext;
mod import_html;
mod mp;
mod new;
mod out;
mod pass;
mod serve;

use crate::component_args::ComponentArgs;
use crate::component_map::ComponentMap;
use crate::dsp_map::DspMap;
use crate::import_base::ImportBase;
use crate::consts::{CONFIG_FILE};
use crate::helpers::version::version;
use crate::serve::serve;
use crate::new::new;
use crate::pass::pass;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let import_base = ImportBase::new();

    if args.len() == 1 {
        let ne = "cream new {project_name} - Create a new project";
        let build = "cream make - Build your project";
        let serve = "cream serve - Serve or run your project";
        let vers = "cream version - Output's cream's version that is currently installed";

        let inst = format!("{ne}\n{build}\n{serve}\n{vers}");
        println!("{inst}");
    } else {
        let mut map;

        match args[1].as_str() {
            "new" => new(args.get(2).expect("Project name not provided")),
            "make" => {
                map = DspMap::new();
                map.load(CONFIG_FILE);
                let mut comp_map = ComponentMap::new(ComponentArgs::new(map, import_base));
                router::router(&mut comp_map);
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
