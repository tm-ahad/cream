mod at_temp;
mod brace_pool;
mod collect_scope;
mod comment;
mod component;
mod component_args;
mod component_markup;
mod consts;
mod dsp_map;
mod escape_string;
mod extract_component;
mod gen_id;
mod helpers;
mod id_gen;
mod import_base;
mod import_lib;
mod import_npm;
mod import_script;
mod input;
mod js_lib;
mod matcher;
mod mp;
mod new;
mod out;
mod pass;
mod quote_base;
mod remove;
mod replacement_flag;
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

use crate::dsp_map::DspMap;
use crate::import_base::ImportBase;
use crate::state_base::_StateBase;
use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use crate::transpiler::transpile;
use crate::consts::{CONFIG_FILE, SPACE};
use crate::new::new;
use crate::pass::pass;
use std::env;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();

    let import_base = ImportBase::new();

    if args.len() == 1 {
        let ne = "cream new {project_name} - Create a new project";
        let build = "cream make - Build your project";

        let inst = format!("{ne}\n{build}");
        println!("{inst}");
    } else {
        let mut map;

        match args[1].as_str() {
            "new" => new(args.get(2).expect("Project name not provided")),
            "make" => {
                map = DspMap::new();
                map.load(CONFIG_FILE);

                transpile(state_base, import_base, &map);

                match map.get("pre_make") {
                    Some(c) => {
                        let mut com = c.split(SPACE).collect::<Vec<&str>>();
                        com.retain(|x| !x.is_empty());

                        if !com.is_empty() {
                            let a = match Command::new(com[0]).args(com[1..].to_vec()).output() {
                                Ok(e) => e.stdout,
                                Err(e) => {
                                    StdErr::exec(OSError, &e.to_string());
                                    Vec::new()
                                }
                            };

                            println!("{}", String::from_utf8_lossy(&a));
                        }
                    }
                    None => pass(),
                }
            }
            &_ => pass(),
        }
    }
}
