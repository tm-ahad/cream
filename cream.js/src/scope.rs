use crate::{state::_state, state_base::_StateBase};
use rusty_v8::{Script, self as v8};
use v8::{ContextScope, HandleScope};
use std::process::Command;
use std::collections::HashMap;

pub fn parse_scope(script: &mut str, ptr: &mut HashMap<usize, String>, scope: &mut ContextScope<HandleScope>) -> String {
    let mut c_script = script.replace(';', "");
    
    let mut com = Command::new("node");

    let out_bytes = match com.args(["/home/ahad/.cream/tools/regexp/main.js", &format!("'{c_script}'")])
        .output() {
            Ok(o) => o.stdout,
            Err(e) => panic!("{e}"),
        };

    let mut out = String::new();

    for bytes in out_bytes {
        out.push(bytes as char)
    }

    let sections = out.split('#').collect::<Vec<&str>>();
    
    for (section_id, section) in (0_u8..).zip(sections) {
        for matches in section.split('\n') {
            if !matches.is_empty() {
                let pair = matches.split('$').collect::<Vec<&str>>();

                let start = pair[0].parse::<usize>()
                    .unwrap();

                let end = pair[1].parse::<usize>()
                    .unwrap();

                let s = &script[start..end];

                if section_id > 0 {
                    let inner = &s[start + 1..s.len() - 1];
                    let mut idx = 0;

                    while &inner[idx..idx] != "{" {
                        idx += 1;
                    }

                    let lines  = inner.lines();
                    let mut parsed_inner = inner[idx..].to_string();

                    let mut state_base = _StateBase::new();

                    let code = v8::String::new(scope, &parsed_inner).unwrap();
                    let script = Script::compile(scope, code, None).unwrap();

                    let _ = script.run(scope).unwrap();

                    _state(&mut parsed_inner, &mut state_base, scope);

                    for line in lines {
                        parsed_inner.push_str(&format!("{}.cam()\n", line))
                    }
                } else {
                    let inner = &s[start + 1..s.len() - 1];

                    ptr.insert(start, String::from(inner));
                }

                c_script.replace_range(start-1..end-2, "");
            }
        }
    }

    c_script

}

pub fn scopify(script: &mut String, map: &HashMap<usize, String>) {
    for (k, v) in map.iter() {
        script.insert_str(*k, v)
    }
}
