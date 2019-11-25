use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("\n\nType 'http://127.0.0.1:7878/' into your browser!");
    println!("Type 'http://127.0.0.1:7878/slow' to simulate a slow request!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: create threads for each connection
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let slow = b"GET /slow HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
    } else if buffer.starts_with(slow) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/slow.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "assets/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
