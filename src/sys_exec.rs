use std::process::Command;

pub fn sys_exec(command: String) {
    let mut split = command.split_whitespace();
    let prog = split.next().unwrap();

    let v: Vec<&str> = split.collect();

    let mut cmd = Command::new(prog);
    cmd.args(v);

    let o = match cmd.output() {
        Ok(t) => t,
        Err(_) => return,
    };

    let mut string_output = String::new();
    let chars = o.stdout;

    for ascii in chars {
        string_output.push(ascii as char);
    }
}
