use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8890").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_path) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n",
            "data/resp.json",
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n",
            "data/resp.json",
        )
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "data/404.html")
    };
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
