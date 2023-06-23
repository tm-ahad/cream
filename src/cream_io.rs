use std::collections::HashMap;
use std::fs::read_to_string;
use crate::channel::Channel;
use crate::compiler_enum::Compiler;
use crate::config::Config;

pub struct Catch(HashMap<String, String>, Vec<Channel>);

pub struct CreamIO {
    config: Config,
    catch: Catch,
}

impl CreamIO {
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn read_src(&mut self) -> &String {
        let lang = self.config.get("lang")
            .unwrap_or(&String::from("js"));
        let path = format!("./build/app.{lang}");

        match self.catch.0.get(&path) {
            Some(s) => s,
            None => {
                let src = read_to_string(&path)
                    .unwrap_or_else(|e| panic!("{e}"));

                self.catch.0.insert(path, src);

                &src
            }
        }
    }

    pub fn set_chans(&mut self) {
        self.catch.1 = vec![
            Channel::new(String::from("./build/.$.js")),
        ]
    }

    pub fn find_chan(&self, typ: Compiler) -> Option<&Channel> {
        let ext = &self.config.lang;

        let path = match typ {
            Compiler::Input => "./build/.$.js",
            Compiler::Output => format!("./build/.$.{lang}")
        };
        let chans = &self.catch.1;

        for c in chans {
            if c.path() == path {
                return Some(c);
            }
        }

        None
    }
}