mod collect_scope;
mod comment;
mod component;
mod component_args;
mod component_markup;
mod consts;
mod dsp_map;
mod extract_component;
mod gen_id;
mod helpers;
mod id_gen;
mod import_base;
mod import_lib;
mod import_npm;
mod import_script;
mod input;
mod javascript_lib;
mod matcher;
mod mp;
mod new;
mod out;
mod pass;
mod quote_base;
mod remove;
mod router;
mod scope;
mod script_module;
mod state;
mod state_base;
mod std_err;
mod sys_exec;
mod template;
mod template_type;
mod transpile_component;
mod transpile_to_js;
mod transpiler;
mod udt;
mod var_not_allowed;
mod import_component;
mod parsable_format;
mod import_ext;
mod import_template;
mod import_html;
mod serve;

use crate::dsp_map::DspMap;
use crate::import_base::ImportBase;
use crate::state_base::_StateBase;
use crate::transpiler::transpile;
use crate::consts::{CONFIG_FILE};
use crate::helpers::version::version;
use crate::serve::serve;
use crate::new::new;
use crate::pass::pass;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();
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
                transpile(state_base, import_base, &map);
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
