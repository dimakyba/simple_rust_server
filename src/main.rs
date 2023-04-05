use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let html_content = fs::read_to_string
        ("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        html_content.len(),
        html_content
    );
    // println!("{}", html_content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // )
}

/* HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body

ex: HTTP/1.1 200 OK\r\n\r\n
 */
