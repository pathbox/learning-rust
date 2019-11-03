use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap(); // 数据读取到buffer

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


