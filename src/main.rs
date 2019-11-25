use rust_workshop::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

/// Creates a Web Server listening on port `7878` and passes connections to
/// `handle_connection()`
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("\n\nType 'http://127.0.0.1:7878/' into your browser!");
    println!("Type 'http://127.0.0.1:7878/slow' to simulate a slow request!");

    // Creates a pool of 5 workers
    // Note: try running with a ThreadPool of size 0 and see what happens!
    let pool = ThreadPool::new(5).unwrap();

    // Iterate over various socket connections
    // Note: `TcpListener` will never generate a `None` value in the iterator
    //       This results in the main thread looping forever.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO #4: create threads for each connection
        // Hint: Make sure to use the `ThreadPool`!
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Read the browser request and send back the proper response
/// Valid Requests:
///     '/' => Hello response
///     '/slow' => Slow hello response (simulates cpu-intensive page)
/// Invalid Requests => 404 response
fn handle_connection(mut stream: TcpStream) {
    // Read browser request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Hardcoded valid request headers
    let get = b"GET / HTTP/1.1\r\n";
    let slow = b"GET /slow HTTP/1.1\r\n";

    // Naive parsing of GET request generates a tuple of response header
    // and path to response contents file
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
    } else if buffer.starts_with(slow) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/slow.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "assets/404.html")
    };

    // write reponse header and contents to stream for browser to display
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
