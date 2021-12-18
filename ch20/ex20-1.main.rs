// 예제 20-1 유입 스트림은 대기하고 스트림을 수신하면 메시지 출력

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
	let stream = stream.unwrap();

	println!("연결됨");
    }
}
