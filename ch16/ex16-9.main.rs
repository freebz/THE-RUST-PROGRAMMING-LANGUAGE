// 예제 16-9 val을 채널에 전달한 후 사용하려고 시도

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
	let val = String::from("안녕하세요");
	tx.send(val).unwrap();
	println!("val = {}", val);
    });

    let received = rx.recv().unwrap();
    println!("수신: {}", received);
}
