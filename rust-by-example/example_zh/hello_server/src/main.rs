use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

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
    let mut buffer = [0; 512]; // 0 开始  512 字节长度

    stream.read(&mut buffer).unwrap(); // 数据读取到buffer

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let contents = fs::read_to_string("hello.html").unwrap(); // hello.html在项目根目录开始找

    // let response = "HTTP/1.1 200 OK\r\n\r\n"; // 最简单的HTTP 文本 response Status Code

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    //  if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();

    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     // 其他请求
    // }

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


