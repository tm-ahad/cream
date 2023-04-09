mod at_html;
mod collect_gen;
mod compiler;
mod component;
mod gen_id;
mod new;
mod scope;
mod state;
mod state_base;
mod template;
mod std_err;
mod var_not_allowed;
mod react_op;
mod v8_parse;
mod pass;
mod cpu_error;

use crate::state_base::_StateBase;
use crate::compiler::compile;
use crate::new::new;
use crate::pass::pass;
use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();

    match args[1].as_str() {
        "new" => new(args.get(2).expect("Project name not provided")),
        "build" => compile(args.get(2).expect("Project name not prvided"), state_base),
        "start" => {
            compile(args.get(2).expect("Project name not provided"), state_base);
            let mut comm = Command::new("./main");

            comm.arg(format!(
                "./{}/build/index.html",
                args.get(2).expect("Project name not provided")
            ))
            .exec();
        }
        _ => pass()
    }
}
