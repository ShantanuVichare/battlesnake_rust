use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Server started");
    
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldn't Bind to port");

    for (id, stream) in listener.incoming().enumerate() {
        let stream = stream.expect("Connection failed");
        println!("Connection established!");
        
        thread::spawn(move || {
            handle_connection(id, stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(id: usize, mut stream: TcpStream) {
    let mut buffer = Vec::new();

    stream.read_to_end(&mut buffer).expect("Failed to read stream buffer");

    fs::write(format!("requests/req_{}.txt",id), buffer).expect("File writing error");
    // stream.read(&mut buffer).expect("Failed to read stream buffer");


    // let get = b"GET / HTTP/1.1\r\n";
    // let sleep = b"GET /sleep HTTP/1.1\r\n";

    // let (status_line, html_file) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    // } else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(5));
    //     ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    // };
    
    // let contents = fs::read_to_string(html_file).expect("Failed to load HTML file");
    // let response = format!("{}{}", status_line, contents);

    stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("Couldn't write response to stream");
    stream.flush().expect("Stream buffer failed to flush");
}


