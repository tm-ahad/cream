mod browser_;
mod collect_gen;
mod compiler;
mod component;
mod new;
mod scope;
mod state;
mod state_base;
mod std_err;
mod template;

use crate::state_base::_StateBase;
use compiler::compile;
use new::new;
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
            //compile(args.get(2).expect("Project name not provided"), state_base, );
            let mut comm = Command::new("./main");

            comm.arg(format!(
                "./{}/build/index.html",
                args.get(2).expect("Project name not provided")
            ))
            .exec();
        }
        _ => {}
    }
}
