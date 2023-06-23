use crate::mp::Mp;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::compiler_enum::Compiler;

pub struct Input<'a>(pub &'a String, pub &'a String);
pub struct Channel(String, File);

impl Channel {
    pub fn path(&self) -> &String {
        &self.0
    }

    pub fn new(path: String) -> Channel {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap_or_else(|e| panic!("{e}"));

        Channel(path, file)
    }

    pub fn write(&mut self, data: Input) {
        let binding = Mp::encode_inp(data);
        let bytes = binding.as_bytes();

        match self.0.write(bytes) {
            Ok(_) => {},
            Err(e) => panic!("{e}")
        }
    }

    pub fn read(&mut self) -> String {
        let mut bytes: [u8; 4294967295] = [0; 4294967295];

        match self.0.read(&mut bytes) {
            Ok(_) => {
                let mut res = String::new();
                for b in bytes {
                    res.push(b as char)
                }
                self.0.set_len(0)
                    .unwrap_or_else(|e| panic!("File: ./build/mp.chan; err: {}", e));
                res
            },
            Err(e) => panic!("{e}")
        }
    }
}