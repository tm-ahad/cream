use std::process::Command;

pub fn sys_exec(command: String) {
    let mut args = command.split_whitespace();

    let program = args.next().unwrap();
    let args = args.collect::<Vec<&str>>();
    let mut cmd = Command::new(program);

    for arg in args {
        cmd.arg(arg);
    }
}
