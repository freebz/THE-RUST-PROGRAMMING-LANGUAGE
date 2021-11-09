// 예제 16-10 여러 메시지를 1초 간격으로 보내기

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
	let vals = vec![
	    String::from("자식 스레드가"),
	    String::from("안녕하세요"),
	    String::from("라고"),
	    String::from("인사합니다."),
	];

	for val in vals {
	    tx.send(val).unwrap();
	    thread::sleep(Duration::from_secs(1));
	}
    });

    for received in rx {
	println!("수신: {}", received);
    }
}
