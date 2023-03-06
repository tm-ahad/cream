mod new;
mod collect_gen;
mod compiler;

use std::env;
use new::new;
use compiler::compile;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    match args[1].as_str() {
        "new" => {
            new(args.get(2)
                .expect("Project name not prvided"))
        }
        "build" => {
            compile(args.get(2)
                .expect("Project name not prvided"))
        }
        _ => {}
    }
}
