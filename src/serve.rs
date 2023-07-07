use crate::config::Config;
use std::io::{Read, Write};
use std::{net::TcpListener, fs::read_to_string};

struct Buffer;
struct Request;

impl Buffer {
    pub fn read(buffer: &[u8], refer: &mut String) {
        for byte in buffer {
            refer.push(*byte as char)
        }
    }
}

impl Request {
    pub fn path(req: String) -> Option<String> {
        if let Some(i) = req.find(' ') {
            let mut idx = i+1;

            while &req[idx..idx+1] != " " {
                idx += 1;
            };

            Some(req[i+1..idx].to_string())
        } else {None}
    }
}

pub fn serve(map: Config) {
    let port = map.get("port")
        .unwrap_or_else(|| panic!("Port not found on config.dsp"));

    let error = read_to_string("./build/error.html")
        .unwrap_or(String::from("<pre style=\"word-wrap: break-word; white-space: pre-wrap;\">404 page not found</pre>"));

    let host = map.get("host")
        .unwrap_or_else(|| panic!("Host not found on config.dsp"));

    let server = TcpListener::bind(format!("{host}:{port}"))
        .unwrap();

    let content = &read_to_string(_app_html)
        .unwrap_or(error.clone());

    for stream in server.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer: [u8; 2048] = [0; 2048];

        let _ = stream.read(&mut buffer);
        let mut req = String::new();

        Buffer::read(&buffer, &mut req);
        let mut path = Request::path(req)
            .unwrap_or(String::from("/"));

        let empty = &String::new();

        let static_dir = map.get("static_dir")
            .unwrap_or(empty);

        let static_dir_render = map.get("static_dir_render")
            .unwrap_or(empty);

        let _app_html = map.get("_app_html")
            .unwrap();

        let len = static_dir_render.len();
        let is_not_main_path = path.starts_with(static_dir_render) && 
            !static_dir.trim().is_empty();

        let mut resp_type = "HTTP/1.1 200 OK\r\n";

        let content = if is_not_main_path {
            path.replace_range(..len, static_dir);

            match read_to_string(path) {
                Ok(c) => c,
                Err(_) => {
                    resp_type = "HTTP/1.1 404 NotFound\r\n";
                    error.clone()
                }
            }
        } else {
            content
        };

        let _ = stream.write(
            format!("{resp_type}Content-Length: {}\r\n\r\n{content}", content.len())
            .as_bytes()
        );
    }

}