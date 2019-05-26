use std::fs;
use std::io::prelude::*;
use std::net::{
    TcpListener,
    TcpStream,
};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:40601").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        handle_connections(stream);
    }
}

fn handle_connections(mut stream : TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // index page
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("server.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else { // 404
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let response = format!("{}", status_line);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
