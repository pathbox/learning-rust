use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
struct ThreadPool;

impl ThreadPool {
    fn new(size: u32) -> ThreadPool { ThreadPool }
    fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {}
}

fn main() {
    let listener = TcpStream::bind("127.0.0.1:8989").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

         pool.execute(|| {
             handle_connection(stream);
         });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512]; // 0 开始  512 字节长度

    stream.read(&mut buffer).unwrap(); // 数据读取到buffer

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let contents = fs::read_to_string("hello.html").unwrap(); // hello.html在项目根目录开始找

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

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
