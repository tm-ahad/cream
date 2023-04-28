mod at_html;
mod collect_gen;
mod compiler;
mod component;
mod new;
mod state;
mod state_base;
mod template;
mod std_err;
mod var_not_allowed;
mod v8_parse;
mod pass;
mod std_out;
mod input;
mod dsp_parser;
mod get_prop;
mod merge_js;
mod id_gen;
mod js_lib;
mod js_module;
mod import_lib;
mod import_script;
mod import_base;

use crate::state_base::_StateBase;
use crate::compiler::compile;
use crate::new::new;
use crate::pass::pass;
use crate::std_out::std_out;
use crate::dsp_parser::dsp_parser;
use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use crate::id_gen::IdGen;
use crate::merge_js::merge_js;
use crate::import_base::ImportBase;
use std::env;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();

    let import_base = ImportBase::new();

    if args.len() == 1 {
        let ne = "ntc new {project_name} - Create a new project";
        let build = "ntc build {project_name} - Build your project";
        let start = "serve (not nts serve) - serve your project\n";

        let inst = format!("{ne}\n{build}\n{start}");

        std_out(inst.as_str())
    } else {
        let map;
        IdGen::init();

        match args[1].as_str() {
            "new" => new(args.get(2).expect("Project name not provided")),
            "build" => {
                map = dsp_parser("./config.dsp");

                compile(state_base, import_base, map.clone());

                match map.get("pre_build") {
                    Some(c) => {
                        let mut com = c.split(' ').collect::<Vec<&str>>();

                        com.retain(|x| !x.is_empty());

                        if !com.is_empty() {
                            let a  = match Command::new(com[0])
                                .args(com[1..].to_vec())
                                .output() {
                                Ok(e) => e.stdout,
                                Err(e) => {
                                    StdErr::exec(OSError, &e.to_string());
                                    Vec::new()

                                }
                            };

                            std_out(&String::from_utf8_lossy(&a));
                        }
                    }
                    None => pass()
                }

                merge_js(map);
            },
            &_ => {}
        }
    }

}
