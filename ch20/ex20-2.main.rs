// 예제 20-2 TcpStream을 읽어 데이터 출력

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
	let stream = stream.unwrap();

	handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("요청: {}", String::from_utf8_lossy(&buffer[..]));
}
