mod at_gen_id;
mod at_html;
mod brace_pool;
mod collect_scope;
mod compiler;
mod component;
mod config;
mod expected;
mod id_gen;
mod import_base;
mod import_lib;
mod import_npm;
mod import_script;
mod input;
mod is_byte_in_str;
mod js_lib;
mod js_module;
mod matcher;
mod mp;
mod new;
mod pass;
mod scope;
mod serve;
mod state;
mod state_base;
mod std_err;
mod std_out;
mod sys_exec;
mod template;
mod udt;
mod v8_parse;
mod var_not_allowed;

use crate::compiler::compile;
use crate::config::Config;
use crate::id_gen::IdGen;
use crate::import_base::ImportBase;
use crate::new::new;
use crate::pass::pass;
use crate::serve::serve;
use crate::state_base::_StateBase;
use crate::std_err::ErrType::OSError;
use crate::std_err::StdErr;
use crate::std_out::std_out;
use std::env;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();

    let import_base = ImportBase::new();

    if args.len() == 1 {
        let ne = "cream new {project_name} - Create a new project";
        let build = "cream make - Build your project";
        let start = "serve (not cream serve) - serve your project\n";

        let inst = format!("{ne}\n{build}\n{start}");

        std_out(&inst)
    } else {
        let mut map;

        match args[1].as_str() {
            "new" => new(args.get(2).expect("Project name not provided")),
            "make" => {
                map = Config::new();
                map.load(String::from("./config.dsp"));

                compile(state_base, import_base, &map);

                match map.get("pre_make") {
                    Some(c) => {
                        let mut com = c.split(' ').collect::<Vec<&str>>();

                        com.retain(|x| !x.is_empty());

                        if !com.is_empty() {
                            let a = match Command::new(com[0]).args(com[1..].to_vec()).output() {
                                Ok(e) => e.stdout,
                                Err(e) => {
                                    StdErr::exec(OSError, &e.to_string());
                                    Vec::new()
                                }
                            };

                            std_out(&String::from_utf8_lossy(&a));
                        }
                    }
                    None => pass(),
                }
            }
            "serve" => {
                map = Config::new();
                map.load(String::from("./config.dsp"));

                serve(map)
            }
            &_ => pass(),
        }
    }
}
