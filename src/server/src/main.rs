use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

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

    let reponse = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        String::from("{ \"hello\": \"Rust\" }")
    );
    println!("{}", reponse);
    stream.write(reponse.as_bytes()).unwrap();
    stream.flush().unwrap();
}
