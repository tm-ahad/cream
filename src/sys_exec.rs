use crate::std_out::std_out;
use std::process::Command;

pub fn sys_exec(command: String) {
    let mut split = command.split_whitespace();
    let prog = split.next().unwrap();

    let v: Vec<&str> = split.collect();

    let mut cmd = Command::new(prog);
    cmd.args(v);

    let o = cmd.output().unwrap_or_else(|e| panic!("{}", e));

    let mut string_output = String::new();
    let chars = o.stdout;

    for ascii in chars {
        string_output.push(ascii as char);
    }

    std_out(&string_output);
}
