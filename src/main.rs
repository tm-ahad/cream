mod new;
mod collect_gen;
mod compiler;
mod state_base;
mod state;
mod std_err;

use std::env;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use new::new;
use compiler::compile;
use crate::state_base::_StateBase;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let state_base = _StateBase::new();

    match args[1].as_str() {
        "new" => {
            new(args.get(2)
                .expect("Project name not prvided"))
        }
        "build" => {
            compile(args.get(2)
                .expect("Project name not prvided"), state_base)
        }
        "start" => {
            compile(args.get(2).expect("Project name not prvided"), state_base);

            let name = args.get(2)
                .expect("Project name not prvided");
            let port = &"8871".to_string();

            let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                handle_connection(stream, name.clone());
            }

            fn handle_connection(mut stream: TcpStream, name: String) {
                let buf_reader = BufReader::new(&mut stream);
                let request_line = buf_reader.lines().next().unwrap().unwrap();

                if request_line == "GET / HTTP/1.1" {
                    let status_line = "HTTP/1.1 200 OK";
                    let contents = read_to_string(format!("./{}/build/index.html", name)).expect("app.js nor found");

                    let length = contents.len();

                    let response = format!(
                        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
                    );

                    stream.write_all(response.as_bytes()).unwrap();
                }
            }

        }
        _ => {}
    }
}
