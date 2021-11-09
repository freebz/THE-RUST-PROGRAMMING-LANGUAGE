// 예제 16-7 자식 스레드로 tx를 옮긴 후 '안녕하세요'라는 문자열 보내기

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
	let val = String::from("안녕하세요");
	tx.send(val).unwrap();
    });
}
