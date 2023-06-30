use crate::mp::Mp;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read, Seek, SeekFrom};

pub struct Input<'a>(pub &'a String, pub &'a String);
pub struct Channel(File);

impl Channel {
    pub fn new(path: String) -> Channel {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)
            .unwrap_or_else(|e| panic!("{e}"));

        Channel(file)
    }

    pub fn write(&mut self, data: Input) {
        let binding = Mp::encode_inp(data);
        let bytes = binding.as_bytes();

        match self.0.write(bytes) {
            Ok(_) => {},
            Err(e) => panic!("{e}")
        }

        self.0.seek(SeekFrom::Start(0))
            .unwrap_or_else(|e| panic!("{e}"));
    }

    pub fn read(&mut self) -> String {
        let mut content = String::new();

        match self.0.read_to_string(&mut content) {
            Ok(_) => content,
            Err(e) => panic!("{}", e)
        }
    }
}