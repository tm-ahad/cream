use crate::sys_exec::sys_exec;
use crate::config::Config;
use crate::{state::_state, state_base::_StateBase};
use rusty_v8::{Script, self as v8};
use v8::{ContextScope, HandleScope};
use std::fs::{read_to_string, write};
use std::process::Command;
use std::collections::HashMap;
use crate::channel::{Channel, Input};

pub fn parse_scope(script: &mut str, ptr: &mut HashMap<usize, String>, scope: &mut ContextScope<HandleScope>) -> String {
    let mut c_script = script.replace(';', "");
    let mut chan = Channel::INSTANCE;

    chan.write(Input::PreDefinedMatcher(c_script.clone()));
    sys_exec(String::from("node /home/ahad/.cream/tools/regexp/main.js"));

    let out = chan.read();

    let sections = out.split('#').collect::<Vec<&str>>();
    
    for (section_id, section) in (0_u8..).zip(sections) {
        for matches in section.split('\n') {
            if !matches.is_empty() {
                let pair = matches.split('$').collect::<Vec<&str>>();

                let start = pair[0].parse::<usize>()
                    .unwrap();

                let end = pair[1].parse::<usize>()
                    .unwrap();

                let s = &script[start..end+3]; 

                if section_id > 0 {
                    let mut idx = 0;

                    while &s[idx..idx+1] != "{" {
                        idx += 1;
                    }

                    let mut parsed_inner = s[idx..].to_string();

                    let cl = parsed_inner.clone();
                    let lines  = cl.lines();

                    let mut state_base   = _StateBase::new();

                    let code = v8::String::new(scope, &parsed_inner).unwrap();
                    let script = Script::compile(scope, code, None).unwrap();

                    let _ = script.run(scope).unwrap();

                    _state(&mut parsed_inner, &mut state_base, scope);

                    for line in lines {
                        parsed_inner.push_str(&format!("{}.cam()\n", line))
                    }
                } else {
                    let mut idx = 0;

                    while &s[idx..idx+1] != "{" {
                        idx += 1;
                    }

                    let parsed_inner = s[idx+1..].to_string();

                    ptr.insert(start-1, parsed_inner);
                }

                c_script.replace_range(start-1..end-2, "");
            } 
        }
    }

    c_script
}

pub fn scopify(script: &mut String, map: &HashMap<usize, String>, config: &Config) {
    for (k, mut v) in map.iter() {
        write(format!("./build/.$.{}", config.expect("lang")), v)
            .unwrap_or_else(|e| panic!("{e}"));

        sys_exec(format!("{} ./build/.$.{}", config.expect("build"), config.expect("lang")));
        let res = read_to_string("./build/.$.js")
            .unwrap();

        v = &res;

        script.insert_str(*k, v)
    }
}
