// 예제 16-8 '안녕하세요'라는 메시지를 주 스레드에서 수신해서 출력

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
	let val = String::from("안녕하세요");
	tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("수신: {}", received);
}
