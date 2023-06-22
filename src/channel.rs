use crate::mp::Mp;
use std::fs::File;
use std::io::{Read, Write};

pub struct Input<'a>(pub &'a String, pub &'a String);
pub struct Channel(File);

impl Channel {
    pub fn new() -> Channel {
        Channel(match File::open("./build/mp.chan") {
            Ok(f) => f,
            Err(e) => panic!("{e}")
        })
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
                res
            },
            Err(e) => panic!("{e}")
        }
    }
}