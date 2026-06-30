use toml::Table;

use crate::config::Config;
use crate::consts::ENTRY_FILE;
use crate::input::std_input;
use crate::std_err::{ErrType::OSError, StdErr};
use crate::helpers::create_file::create_file;
use crate::helpers::format_colored::format_colored;
use std::collections::HashMap;
use std::fs::{File, create_dir, create_dir_all};
use std::io::Write;
use std::process::exit;

pub fn create_project(name: &String) {
    let ok = std_input("Ok to processed (y)? ", "y");

    if ok == "y" || ok == "yes" || ok == "ok" {
        create_dir(format!("./{}", name)).expect("Directory Exists");
        create_dir(format!("./{}/src", name)).expect("Directory Exists");
        create_dir(format!("./{}/build", name)).expect("Directory Exists");

        let mut entry = File::create(format!("./{name}/{ENTRY_FILE}",))
            .unwrap_or_else(|e| {
                StdErr::exec(OSError, &format!("Error creating ./{name}/{ENTRY_FILE}: {e}"));
                exit(1);
            });

        let mut config = create_file(format!("./{name}/config.toml"));
        let inst = format!("{} ✨", format_colored("Done", 0, 255, 0));
        let _ = create_dir_all(format!("./{name}/build/"));

        let initial_config = Config {
            build: vec![ENTRY_FILE.to_string()],
            port: 6767,
            env: HashMap::new(),
            packages: Table::new()
        };

        config
            .write_all(
                toml::to_string(&initial_config)
                .unwrap()
                .as_bytes()
            )
            .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));

        entry.write_all("<h1>Hello World</h1>\n<script name=\"App\"></script>\n".as_bytes())
            .unwrap_or_else(|e| StdErr::exec(OSError, &e.to_string()));
        println!("{inst}");
    }
}
