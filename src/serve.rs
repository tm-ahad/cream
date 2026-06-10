use crate::dsp_map::DspMap;
use crate::std_err::ErrType;
use std::collections::BTreeMap;
use std::path::{Component, Path};
use httparse::Request;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::TcpListener;

static CACHE: BTreeMap<String, String> = BTreeMap::new();

pub fn read(path: &str) -> Result<String, Error> {
    if CACHE.contains_key(path) {
        Ok(CACHE.get(path).unwrap().to_string())
    } else {
        fs::read_to_string(path)
    }
}

fn serve_file(path: String, err: String) -> (String, String) {
    let path = if path.is_empty() {"/"} else {&path};
    let _path = path.replace("/", ":");
    match fs::read_to_string(format!("./build/{}", _path)) {
        Ok(content) => ("HTTP/1.1 200 OK\r\n".to_string(), content),
        Err(_) => ("HTTP/1.1 404 Not Found\r\n".to_string(), err),
    }
}

pub fn serve(map: DspMap) {
    let port = map.get("port").unwrap_or_else(|| panic!("Port not found on config.dsp"));
    let host = format!("127.0.0.1:{}", port);
    let server = TcpListener::bind(host.clone()).unwrap();

    let err = fs::read_to_string("./build/error").unwrap();
    println!("crème brûlée servie sur http://{host}");

    for mut stream in server.incoming().flatten() {
        let mut buffer = [0; 2048];

        if stream.read(&mut buffer).is_ok() {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = Request::new(&mut headers);

            if let Ok(request) = req.parse(&buffer) {
                if request.is_complete() {
                    let path = req.path.unwrap_or("/");

                    let static_dir_render = map.get("static_dir_render")
                        .unwrap_or_else(|| panic!("Static dir render not found"));

                    let path = Path::new(path)
                        .components()
                        .map(|c| match c {
                            Component::Normal(os_str) => os_str.to_string_lossy().to_string(),
                            _ => String::new(), // ignore RootDir, Prefix, etc.
                        })
                        .collect::<Vec<String>>()
                        .join("/");
                    
                    let (resp_type, content) = if path.starts_with(static_dir_render) {
                        let static_dir = map.get("static_dir").unwrap_or_else(|| panic!("Static dir not found"));
                        let len = static_dir_render.len();
                        let mut file_path = static_dir.to_string();
                        file_path.insert_str(0, "./");
                        file_path.push('/');
                        file_path.push_str(&path[len..]);

                        match read(&file_path) {
                            Ok(content) => ("HTTP/1.1 200 OK\r\n".to_string(), content.to_string()),
                            Err(_) => serve_file(path, err.clone()),
                        }
                    } else {
                        serve_file(path.to_string(), err.clone())
                    };

                    let response = format!("{}Content-Length: {}\r\n\r\n{}", resp_type, content.len(), content);
                    let _ = stream.write(response.as_bytes());
                }
            } else {
                let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
                let _ = stream.write(response.as_bytes());
            }
        }
    }
}
